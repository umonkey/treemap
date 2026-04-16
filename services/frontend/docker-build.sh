#!/bin/sh
# This script is used to build the static front-end for including
# in the distributed Docker image.

set -e
npm ci --loglevel verbose
npm run build
