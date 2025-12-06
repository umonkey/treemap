#!/bin/sh
# Stitch photos taken by a drone into a single GeoTIFF.
# Your input is a bunch of JPEG files, and the output is a single GeoTIFF file.

set -e

if [ -z "$1" ]; then
    echo "Usage: $0 *.jpg"
    exit 1
fi

if [ -d datasets ]; then
    echo "Oops, folder datasets already exists. Please delete first."
    exit 1
fi

echo "Copying files to datasets/project/images..."
mkdir -p datasets/project/images
while [ -n "$1" ]; do
    cp "$1" "datasets/project/images/$1"
    shift
done

echo "Executing the stitcher ..."
docker run -ti --rm -v "$PWD/datasets":/datasets docker.io/opendronemap/odm --project-path /datasets --split=100 --fast-orthophoto --skip-3dmodel project 2>&1 | tee odm.log

echo "Moving results to local folder ..."
mv datasets/project/odm_report/report.pdf datasets/project/odm_orthophoto/odm_orthophoto.tif ./

echo "Cleaning up ..."
rm -rf datasets

echo "Done. Please keep this output file:"
ls -ldh odm_orthophoto.tif
