#!/bin/sh
# This script runs on the server where the app is deployed.
# It is triggered by a CI/CD job (GitHub Actions) via SSH.
# The goal of this script is to update and gracefully restart the app.

echo $CR_TOKEN | docker login ghcr.io -u $CR_USER --password-stdin

cd ~/treemap
git pull
docker compose -f compose.prod.yaml pull
exec docker compose -f compose.prod.yaml up
