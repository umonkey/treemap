#!/bin/sh
set -e
npm ci --loglevel verbose
npm run build
