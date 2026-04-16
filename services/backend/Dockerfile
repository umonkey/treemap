# Build the target deployment image for "Trees of Yerevan".
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

FROM docker.io/library/rust:1.95-bookworm AS builder
RUN apt-get update && apt-get install -y libssl-dev pkg-config

WORKDIR /app
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build only dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY src src
COPY dev dev
RUN touch src/main.rs
RUN cargo build --release

# STEP 2: Build the Final Image
FROM docker.io/library/debian:bookworm-slim
RUN apt-get update && \
    apt-get install -y sqlite3 supervisor logrotate bash ca-certificates cron && \
    ln -s /usr/bin/sqlite3 /usr/bin/sqlite && \
    ln -s /usr/sbin/cron /usr/sbin/crond && \
    mkdir -p /var/spool/cron/crontabs && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/treemap bin/treemap
COPY docker/rootfs/ /
RUN cp /etc/crontabs/root /var/spool/cron/crontabs/root && chmod 600 /var/spool/cron/crontabs/root
COPY dev/schema-sqlite.sql /app
COPY static /app/static
COPY config.toml.dev /app

ENV RUST_LOG=info,treemap=debug

EXPOSE 8000/tcp
CMD ["/app/startup.sh"]
