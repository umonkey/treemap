#!/bin/sh
rm -rf var/dataset/*.json var/dataset/config.yaml var/dataset/*.ply var/dataset/exif var/dataset/features var/dataset/masks var/dataset/matches/ var/dataset/reports/ var/dataset/*.log

time docker run --rm -v `pwd`/var:/app/var \
    -u 1000:1000 \
    -e "GPX_OFFSET=60.5" \
    -e "MASK_SIZE=0.35" \
    -t treemap-extractor:latest bin/process
