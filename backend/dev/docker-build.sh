#!/bin/sh
# This script builds the application in a Docker container.

apk add --no-cache musl-dev openssl-dev
export RUSTFLAGS=-Ctarget-feature=-crt-static

# Without this, linking fails.
# https://github.com/rust-lang/rust/issues/115450#issuecomment-1717228111
export OPENSSL_DIR=/usr

exec cargo build --release
