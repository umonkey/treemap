# Build the target deployment image.
#
# Note that the image is compiled from pre-build binaries and static files.
# There are separate build steps to produce those, se commends below.

FROM alpine:3.18
LABEL maintainer="hex@umonkey.net"
LABEL org.opencontainers.image.source=https://github.com/umonkey/treemap
LABEL org.opencontainers.image.description="A simple self-container backend and frontend image using an SQLite database."

RUN apk add --no-cache sqlite supervisor

WORKDIR /app

# These files are made with: make build-frontend-docker
COPY frontend/dist /app/static

# These files are made with: make build-backend-docker
COPY backend/target/release/treemap /app/bin/treemap
COPY backend/dev/schema-sqlite.sql /app/schema-sqlite.sql
COPY backend/dev/species.sql /app/species.sql

COPY container/rootfs/ /

ENV RUST_LOG info,treemap=debug
ENV TREEMAP_SQLITE_PATH /app/var/database.sqlite

EXPOSE 8000/tcp
CMD ["/app/startup.sh"]
