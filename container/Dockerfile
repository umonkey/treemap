# (1) Build the backend binary.
FROM docker.io/rust:1.76-alpine3.18 as rust-builder
RUN apk add --no-cache musl-dev openssl openssl-dev openssl-libs-static
COPY backend /app
RUN cd /app && cargo build --release

# (2) Build the frontend static files.
FROM docker.io/node:20-alpine3.18 as node-builder
COPY frontend /app
RUN cd /app && npm install --loglevel verbose && npm run build

# (3) Build the target image.
FROM alpine:3.18
LABEL maintainer="hex@umonkey.net"
LABEL org.opencontainers.image.source=https://github.com/umonkey/treemap
LABEL org.opencontainers.image.description="A simple self-container backend and frontend image using an SQLite database."

RUN apk add --no-cache sqlite supervisor

WORKDIR /app
COPY --from=rust-builder /app/target/release/treemap /app/bin/
COPY --from=rust-builder /app/dev/schema-sqlite.sql /app/schema-sqlite.sql
COPY --from=rust-builder /app/dev/species.sql /app/species.sql
COPY --from=node-builder /app/dist /app/static
COPY container/rootfs/ /

ENV RUST_LOG info,treemap=debug
ENV TREEMAP_SQLITE_PATH /app/var/database.sqlite

EXPOSE 8000/tcp
CMD ["/app/startup.sh"]
