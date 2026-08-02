FROM docker.io/library/debian:bookworm-slim
LABEL org.opencontainers.image.source=https://github.com/umonkey/treemap

# Install ffmpeg and tools needed for AWS CLI installation
RUN apt-get update && \
    apt-get install -y ffmpeg curl unzip ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Install the latest AWS CLI v2
RUN curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip" && \
    unzip awscliv2.zip && \
    ./aws/install && \
    rm -rf awscliv2.zip aws

WORKDIR /app
COPY convert.sh /app/convert.sh

ENTRYPOINT ["/app/convert.sh"]
