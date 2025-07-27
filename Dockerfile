# This file builds the container image for Trees of Yerevan.

# Build the backend.
FROM docker.io/library/rust:1.88-alpine3.22 AS backend
RUN apk add --no-cache musl-dev openssl-dev
ENV RUSTFLAGS=-Ctarget-feature=-crt-static
ENV OPENSSL_DIR=/usr
COPY backend /app
WORKDIR /app
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=target \
    cargo build --release \
    && mkdir -p /app/bin \
    && mv /app/target/release/treemap /app/bin/treemap

# Build the frontend.
FROM docker.io/library/node:24-alpine3.22 AS frontend
ENV VITE_ENVIRONMENT=production
ENV VITE_API_ROOT=/
COPY frontend /app
WORKDIR /app
RUN --mount=type=cache,target=/root/.npm \
    npm config set cache /root/.npm \
    && npm ci && npm run build

# Build the final image.
FROM docker.io/library/alpine:3.22 AS app
LABEL maintainer="hex@umonkey.net"
LABEL org.opencontainers.image.source=https://github.com/umonkey/treemap
LABEL org.opencontainers.image.description="A simple self-contained backend and frontend image using an SQLite database."
RUN apk add --no-cache sqlite supervisor logrotate
WORKDIR /app
COPY --from=backend /app/bin/treemap /app/bin/treemap
COPY --from=frontend /app/build /app/static
COPY backend/dev/schema-sqlite.sql /app/schema-sqlite.sql
COPY backend/dev/species.sql /app/species.sql
COPY container/rootfs/ /
#RUN find /app -type f | xargs ls -ldh
ENV RUST_LOG info,treemap=debug
ENV TREEMAP_SQLITE_PATH /app/var/database.sqlite
EXPOSE 8000/tcp
CMD ["/app/startup.sh"]
