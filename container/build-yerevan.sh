#!/bin/sh
# This script is used to build a single container for the Yerevan Tree Map.
# The container includes both backend and frontend and all files to run the application.
#
# (4) test-frontend
# (5) build-image
# (6) publish-image

set -e

API_ROOT="https://yerevan.treemaps.app/"
VERSION="0.1.3"

# (1) Log in to the Container registry.
if [ -z "$CR_TOKEN" ]; then
  echo "CR_TOKEN is not set. Must be set to the authentication token that can publish to the GitHub Container Registry. Generate one here: <https://github.com/settings/tokens>"
  exit 1
fi

if [ -z "$CR_USER" ]; then
  echo "CR_USER is not set. Cannot continue."
  exit 1
fi

echo $CR_TOKEN | docker login ghcr.io -u $CR_USER --password-stdin

# (2) Prepare the Rust builder Docker image.  That's the image that
# contains the right Rust version with all build time dependencies.
# We don't use the mult-stage build here because we need to keep the
# cache folder between runs.
docker build --tag treemap_rust_builder:latest --file container/Dockerfile.rust-builder .

# (3) Build the backend.
mkdir -p .cache/rust-registry
docker run --rm \
    -v $PWD/backend:/app \
    -v $PWD/.cache/rust-registry:/root/.cargo/registry \
    -w /app \
    treemap_rust_builder:latest \
    sh -c "cargo build --release && chown -R 1000:1000 /usr/local/cargo/registry /app/target"

# (4) Test and build the frontend.
mkdir -p .cache/npm
docker run --rm \
    -v $PWD/frontend:/app \
    -v $PWD/.cache/npm:/root/.npm \
    -w /app \
    -e VITE_API_ROOT="$API_ROOT" \
    -e VITE_SENTRY_AUTH_TOKEN="$SENTRY_AUTH_TOKEN" \
    -e VITE_ENVIRONMENT="production" \
    -e NODE_ENV="production" \
    --network=host \
    --ulimit nofile=5000:5000 \
    node:20-alpine3.18 \
    sh -c "npm ci && npm run build"

# (5) Build the final Docker image.
docker build --tag treemap:$VERSION --file container/Dockerfile.yerevan .

# (6) Publish the Docker image.
docker tag treemap:$VERSION ghcr.io/$CR_USER/treemap:$VERSION
docker tag treemap:$VERSION ghcr.io/$CR_USER/treemap:latest
docker push ghcr.io/$CR_USER/treemap:$VERSION
docker push ghcr.io/$CR_USER/treemap:latest
