#!/bin/sh
# This script runs on the server where the app is deployed.
# It is triggered by a CI/CD job (GitHub Actions) via SSH.
# The goal of this script is to update and gracefully restart the app.
#
# Note that the CI/CD job must set up CR_USER and CR_TOKEN,
# those are needed to authenticate against ghcr.io and pull images.

cd $(dirname $0)

if [ -z "$CR_TOKEN" ]; then
    echo "CR_TOKEN not set, cannot continue."
    exit 1
fi

if [ -z "$CR_USER" ]; then
    echo "CR_USER not set, cannot continue."
    exit 1
fi

echo $CR_TOKEN | docker login ghcr.io -u $CR_USER --password-stdin

git pull
docker compose -f compose.prod.yaml pull
docker compose -f compose.prod.yaml up -d

# Clean-up old images to reclaim disk space.
docker image prune -f
