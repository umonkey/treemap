# --- STAGE 1: The Builder (Fat Image) ---
FROM debian:bookworm AS builder

ARG DEBIAN_FRONTEND=noninteractive

# Copy uv binary
COPY --from=astral/uv:latest /uv /uvx /bin/

# 1. Install build-time dependencies
RUN apt-get update && apt-get install -y \
    git build-essential cmake \
    libeigen3-dev libopencv-dev libceres-dev \
    libgoogle-glog-dev libgflags-dev \
    python3-dev \
    && apt-get clean

# 2. Create a virtual environment at /opt/venv
RUN uv venv /opt/venv
ENV PATH="/opt/venv/bin:$PATH"

# 3. Clone OpenSfM and install it into /opt/venv using uv pip install .
WORKDIR /source
RUN git clone --depth 1 --branch main https://github.com/mapillary/OpenSfM.git . && \
    git submodule update --init --recursive
RUN uv pip install .

# 4. Copy OpenSfM's bin/* scripts to /opt/venv/bin/
RUN cp -vr /source/bin/* /opt/venv/bin/ && \
    chmod +x /opt/venv/bin/opensfm*

# 5. Install app dependencies from pyproject.toml into /opt/venv
WORKDIR /app
COPY pyproject.toml uv.lock ./
RUN uv pip install -r pyproject.toml

# --- STAGE 2: The Final Image (Slim Image) ---
FROM debian:bookworm-slim
LABEL org.opencontainers.image.source=https://github.com/umonkey/treemap

ARG DEBIAN_FRONTEND=noninteractive

# Copy uv binary
COPY --from=astral/uv:latest /uv /uvx /bin/

# 6. Install RUNTIME dependencies ONLY (including ffmpeg)
RUN apt-get update && apt-get install -y --no-install-recommends \
    python3 \
    ffmpeg \
    awscli \
    procps \
    libopencv-core406 \
    libopencv-imgproc406 \
    libopencv-calib3d406 \
    libopencv-imgcodecs406 \
    libopencv-videoio406 \
    libopencv-features2d406 \
    libopencv-flann406 \
    libceres3 \
    libgoogle-glog0v6 \
    libgflags2.2 \
    libatlas3-base \
    liblapack3 \
    libgomp1 \
    libgl1 \
    libglib2.0-0 \
    libimage-exiftool-perl \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

RUN groupadd -g 1000 extractor && \
    useradd -u 1000 -g extractor -s /bin/bash extractor

# 7. Copy the entire virtual environment
COPY --from=builder /opt/venv /opt/venv

# 8. Set environment variables
ENV PATH="/opt/venv/bin:$PATH"
ENV VIRTUAL_ENV="/opt/venv"
ENV PYTHONUNBUFFERED=1
ENV MPLCONFIGDIR="/tmp/matplotlib"

# 9. Copy custom code.
WORKDIR /app
COPY app /app/app
COPY bin /app/bin
COPY pyproject.toml uv.lock /app
COPY config.yaml /app

# 10. Final verification checks
RUN python3 -c "import opensfm; print('OpenSfM Python package successfully installed')" && \
    python3 -c "import av, gpxpy, piexif, PIL, requests; print('App dependencies successfully imported')" && \
    opensfm --help > /dev/null && \
    ffmpeg -version > /dev/null && \
    aws --version > /dev/null

# 11. Set the default command
RUN chmod +x /app/bin/*
CMD ["bin/process"]
