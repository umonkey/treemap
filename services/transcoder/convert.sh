#!/bin/bash
# The main transcoder script.  Does the following:
# (1) Download the original mp4 file
# (2) Optionally calculate gpx offset if track.gpx exists
# (3) Transcode to 360p
# (4) Upload the target mp4 file and video.json if present

set -e

mkdir -p var/dataset

if [ -n "$DATASET_URL" ]; then
    echo "=== Downloading remote dataset ==="
    time aws s3 sync --no-progress "$DATASET_URL/" var/dataset/
fi

if [ ! -f var/dataset/video.mp4 ]; then
    echo "No video.mp4 in the dataset, cannot continue."
    exit 1
fi

echo "=== Listing dataset files ==="
ls -lh var/dataset/*

# (2) Extract video creation_time and set as gpx_offset (video.json is generated here and uploaded in Step 4)
echo "=== Extracting video creation_time for gpx_offset ==="
video_time_str=$(ffprobe -v quiet -select_streams v:0 -show_entries stream_tags=creation_time -of default=noprint_wrappers=1:nokey=1 var/dataset/video.mp4 | tr -d '\r')
if [ -n "$video_time_str" ]; then
    video_sec=$(date -d "$video_time_str" +%s 2>/dev/null || echo "")

    if [ -n "$video_sec" ]; then
        echo "Calculated timestamp: $video_sec"
        echo "{\"creation_time\": $video_sec}" > var/dataset/video.json
    else
        echo "Could not extract the timestamp."
    fi
else
    echo "WARNING: creation_time not found, dump follows."
    ffprobe var/dataset/video.mp4
fi

# (3) Transcode the file
if [ ! -f var/dataset/video-360p.mp4 ]; then
    echo "=== Transcoding the video ==="
    time ffmpeg -i ./var/dataset/video.mp4 -vf "scale=-2:360,format=yuv420p" -c:v libx264 -crf 30 -preset veryfast -movflags +faststart -an /tmp/tmp.mp4
    mv /tmp/tmp.mp4 var/dataset/video-360p.mp4
else
    echo "Transcoded video is there, reusing."
fi

ls -lh var/dataset/*

# (4) Upload the results (target mp4 and video.json if it exists).
if [ -n "$DATASET_URL" ]; then
    echo "=== Uploading results ..."
    time aws s3 sync --no-progress var/dataset/ "$DATASET_URL/"
fi
