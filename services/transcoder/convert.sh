#!/bin/sh
# The main transcoder script.  Does the following:
# (1) Download the original mp4 file
# (2) Transcode to 360p
# (3) Upload the target mp4 file

set -e

if [ -z "$2" ]; then
    echo "Usage: ./convert.sh s3://bucket/source.mp4 s3://bucket/target.mp4"
    exit 1
fi

# (1) Download the original mp4 file.
aws s3 cp "$1" ./source.mp4

# (2) Transcode the file
ffmpeg -i ./source.mp4 -vf "scale=-2:360,format=yuv420p" -c:v libx264 -crf 30 -preset veryfast -movflags +faststart -an target.mp4
ls -lh target.mp4

# (3) Upload the results.
aws s3 cp target.mp4 "$2"
