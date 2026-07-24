# --- STAGE 1: The Builder (Fat Image) ---
FROM debian:bookworm AS builder

ARG DEBIAN_FRONTEND=noninteractive

# 1. Install build-time dependencies
RUN apt-get update && apt-get install -y \
    git build-essential cmake \
    libeigen3-dev libopencv-dev libceres-dev \
    libgoogle-glog-dev libgflags-dev \
    python3-dev python3-pip python3-venv \
    && apt-get clean

# 2. Create a virtual environment
RUN python3 -m venv /opt/opensfm
ENV PATH="/opt/opensfm/bin:$PATH"

# 3. Upgrade pip and install build requirements
RUN pip install --upgrade pip wheel setuptools

# 4. Clone OpenSfM
WORKDIR /source
RUN git clone --depth 1 --branch main https://github.com/mapillary/OpenSfM.git . && \
    git submodule update --init --recursive

# 5. Build and install OpenSfM
RUN pip install .

# 6. Manually install all scripts from bin/ as they depend on each other's proximity
RUN cp -vr /source/bin/* /opt/opensfm/bin/ && \
    chmod +x /opt/opensfm/bin/opensfm*

# --- STAGE 2: The Final Image (Slim Image) ---
FROM debian:bookworm-slim

ARG DEBIAN_FRONTEND=noninteractive

# 7. Install RUNTIME dependencies ONLY
RUN apt-get update && apt-get install -y --no-install-recommends \
    python3 \
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
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# 8. Copy the entire virtual environment
COPY --from=builder /opt/opensfm /opt/opensfm

# 9. Set environment variables
ENV PATH="/opt/opensfm/bin:$PATH"
ENV VIRTUAL_ENV="/opt/opensfm"

# 10. Copy custom code.
WORKDIR /app
COPY app /app/app
COPY bin /app/bin
COPY pyproject.toml uv.lock /app

# Final verification check
RUN python3 -c "import opensfm; print('OpenSfM Python package successfully installed')" && \
    opensfm --help > /dev/null
