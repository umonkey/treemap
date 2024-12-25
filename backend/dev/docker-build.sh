#!/bin/sh
# This script builds the application in a Docker container.

apk add --no-cache musl-dev openssl-dev
export RUSTFLAGS=-Ctarget-feature=-crt-static

# Without this, linking fails.
# https://github.com/rust-lang/rust/issues/115450#issuecomment-1717228111
export OPENSSL_DIR=/usr

cargo build --release

# Some files are owner by root, which makes the cache GitHub action fail.
# Fix this by resetting the ownership to something that tar can handle.
chown -R 1000:1000 /usr/local/cargo/registry
