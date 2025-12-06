#!/bin/sh
set -e

MOSAIC=mosaic.vrt

if [ -f $MOSAIC ]; then
    echo "Mosaic file exists, reusing."
else
    echo "Creating a virtual mosaic..."
    gdalbuildvrt $MOSAIC */*.tif
fi

echo "Creating tiles..."
mkdir -p tiles
gdal2tiles --zoom=2-25 mosaic.vrt ./tiles
