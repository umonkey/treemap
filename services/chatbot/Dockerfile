# Build the production image for the chatbot.

#############################
# PHASE 1: build the chatbot.
#############################

FROM docker.io/library/rust:1.97-bookworm AS builder
RUN apt-get update && apt-get install -y libssl-dev pkg-config

WORKDIR /app
COPY Cargo.toml Cargo.lock ./

# STEP 1: build only dependencies.
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release
RUN rm -rf src

# Now build the main application.
COPY src src
COPY locales locales
RUN touch src/main.rs
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release && \
    cp target/release/chatbot /app/chatbot-bin


#############################
# PHASE 2: build the final image.
#############################

FROM docker.io/library/debian:bookworm-slim
LABEL maintainer="hex@umonkey.net"
LABEL org.opencontainers.image.source=https://github.com/umonkey/treemap
LABEL org.opencontainers.image.description="Chatbot for Trees of Yerevan."

RUN apt-get update && \
    apt-get install -y supervisor ca-certificates procps htop && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/chatbot-bin bin/chatbot
COPY --from=builder /app/locales locales
COPY docker/rootfs/ /

HEALTHCHECK --interval=30s --timeout=5s --start-period=30s --retries=3 \
  CMD supervisorctl status chatbot || exit 1
CMD ["/usr/bin/supervisord", "-nc", "/etc/supervisord.conf"]
