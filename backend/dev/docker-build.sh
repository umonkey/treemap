#!/bin/sh
# This script builds the application in a Docker container.

cargo clippy
cargo test
cargo build --release

# Some files are owner by root, which makes the cache GitHub action fail.
# Fix this by resetting the ownership to something that tar can handle.
chown -R 1000:1000 /usr/local/cargo/registry
