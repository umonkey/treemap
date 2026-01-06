# Build the target deployment image for "Trees of Yerevan".
#
# Note that the image is compiled from pre-build binaries and static files.
# There are separate build steps to produce those, se commends below.

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

ENV RUST_LOG info,treemap=debug

EXPOSE 8000/tcp
CMD ["/app/startup.sh"]
