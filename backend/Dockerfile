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

FROM docker.io/library/rust:1.88-alpine3.22 as builder
RUN apk add --no-cache musl-dev openssl-dev
ENV RUSTFLAGS=-Ctarget-feature=-crt-static
ENV OPENSSL_DIR=/usr
RUN rustup component add clippy

WORKDIR /app
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build only dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY src src
COPY dev dev
RUN touch src/main.rs
RUN find -type f && cargo build --release && find target -type f

# STEP 2: Build the Final Image
FROM docker.io/library/alpine:3.22
RUN apk add --no-cache sqlite supervisor logrotate bash
WORKDIR /app
COPY --from=builder /app/target/release/treemap bin/treemap
COPY docker/rootfs/ /
COPY dev/schema-sqlite.sql /app
COPY static /app/static

ENV RUST_LOG info,treemap=debug

EXPOSE 8000/tcp
CMD ["/app/startup.sh"]
