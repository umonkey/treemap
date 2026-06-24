FROM python:3.11-slim-bookworm

ARG DEBIAN_FRONTEND=noninteractive

# 1. Install OpenSfM runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
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
    ca-certificates \
    curl \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# 2. Install uv
COPY --from=ghcr.io/astral-sh/uv:latest /uv /uvx /bin/

# 3. Copy pre-compiled OpenSfM
COPY --from=ghcr.io/kanachyerevan/opensfm:latest /opt/opensfm /opt/opensfm

WORKDIR /app

# 4. Setup application
COPY pyproject.toml uv.lock README.md ./

# 5. Build application environment
RUN --mount=type=cache,target=/root/.cache/uv \
    uv sync --frozen --no-dev --no-install-project

COPY app app
COPY bin bin
RUN chmod +x bin/*

RUN --mount=type=cache,target=/root/.cache/uv \
    uv sync --frozen --no-dev --no-editable

# 6. Final configuration
ENTRYPOINT []
ENV PATH="/app/.venv/bin:$PATH"
ENV UV_NO_SYNC=1

# 7. Verification
RUN bin/opensfm --help > /dev/null
