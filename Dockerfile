# Build the production image for "Trees of Yerevan".
# The image contains both the backend and frontend.
#
# This image contains both the backend and frontend.
#
# Note that the build process is built in two steps to speed up compilation:
#
# (1) An empty source file src/main.rs is created and compiled.  This makes Rust
# download and compile all third party crates listed in Cargo.lock, and save
# that in the Docker layers, which are reused later.
#
# (2) The actual src/main.rs is being compiled, with all the dependencies ready.
# This means that as long as you only change local source files, they are compiled
# quickly using pre-compiled dependencies.
#
# If you change the dependencies, however, like add or update a crate, the first
# step runs and the dependencies are re-compiled.
#
# This approach saves time drastically, which saves CI/CD resources.

#############################
# PHASE 1: build the backend.
#############################

FROM docker.io/library/rust:1.95-bookworm AS builder
RUN apt-get update && apt-get install -y libssl-dev pkg-config
ENV OPENSSL_DIR=/usr

WORKDIR /app
COPY services/backend/Cargo.toml services/backend/Cargo.lock ./

# STEP 1: build the backend.
# Create a dummy main.rs to build only dependencies.
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release
RUN rm -rf src

# Now build the main application.
COPY services/backend/src src
COPY services/backend/dev dev
RUN touch src/main.rs
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release && \
    cp target/release/treemap /app/treemap-bin


##############################
# PHASE 2: build the frontend.
##############################

FROM docker.io/library/node:22-bookworm AS frontend-builder
WORKDIR /app
COPY services/frontend/package*.json ./
RUN corepack enable && \
    --mount=type=cache,target=/root/.npm \
    npm ci
COPY services/frontend/. .
RUN npm run build


#################################
# PHASE 3: build the final image.
#################################

FROM docker.io/library/debian:bookworm-slim
LABEL maintainer="hex@umonkey.net"
LABEL org.opencontainers.image.source=https://github.com/umonkey/treemap
LABEL org.opencontainers.image.description="A simple self-contained backend and frontend image using an SQLite database."
RUN apt-get update && \
    apt-get install -y sqlite3 supervisor logrotate bash ca-certificates cron && \
    ln -s /usr/bin/sqlite3 /usr/bin/sqlite && \
    ln -s /usr/sbin/cron /usr/sbin/crond && \
    mkdir -p /var/spool/cron/crontabs && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/treemap-bin bin/treemap
COPY services/backend/docker/rootfs/ /
RUN if [ -f /etc/crontabs/root ]; then mkdir -p /var/spool/cron/crontabs && cp /etc/crontabs/root /var/spool/cron/crontabs/root && chmod 600 /var/spool/cron/crontabs/root; fi
COPY services/backend/dev/schema-sqlite.sql /app
COPY services/backend/config.toml.dev /app
COPY --from=frontend-builder /app/build /app/static

ENV RUST_LOG=info,treemap=debug

EXPOSE 8000/tcp
CMD ["/app/startup.sh"]
