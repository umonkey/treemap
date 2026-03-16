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

FROM docker.io/library/rust:1.88-alpine3.22 AS builder
RUN apk add --no-cache musl-dev openssl-dev
ENV RUSTFLAGS=-Ctarget-feature=-crt-static
ENV OPENSSL_DIR=/usr

WORKDIR /app
COPY backend/Cargo.toml backend/Cargo.lock ./

# STEP 1: build the backend.
# Create a dummy main.rs to build only dependencies.
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Now build the main application.
COPY backend/src src
COPY backend/dev dev
RUN touch src/main.rs
RUN cargo build --release


##############################
# PHASE 2: build the frontend.
##############################

FROM docker.io/library/node:22-alpine AS frontend-builder
WORKDIR /app
COPY frontend/package*.json ./
RUN npm ci
COPY frontend/. .
RUN npm run build


#################################
# PHASE 3: build the final image.
#################################

FROM docker.io/library/alpine:3.22
RUN apk add --no-cache sqlite supervisor logrotate bash
WORKDIR /app
COPY --from=builder /app/target/release/treemap bin/treemap
COPY --from=frontend-builder /app/build /app/static
COPY backend/docker/rootfs/ /
COPY backend/dev/schema-sqlite.sql /app
COPY backend/static /app/static
COPY backend/config.toml.dev /app

ENV RUST_LOG=info,treemap=debug

EXPOSE 8000/tcp
CMD ["/app/startup.sh"]
