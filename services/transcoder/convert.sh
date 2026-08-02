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

if [ ! -f var/dataset/track.gpx ]; then
    echo "No track.gpx in the dataset, cannot continue."
    exit 1
fi

echo "=== Listing dataset files ==="
ls -lh var/dataset/*

# (2) Attempt to download track.gpx and calculate offset if it exists (video.json is generated here and uploaded in Step 4)
echo "=== Calculating gpx_offset ==="
video_time_str=$(ffprobe -v quiet -select_streams v:0 -show_entries stream_tags=creation_time -of default=noprint_wrappers=1:nokey=1 var/dataset/video.mp4)
# Get the time from the first trackpoint, skipping metadata time which is often the file end time.
gpx_time_str=$(grep -m 1 '<trkpt' -A 10 ./var/dataset/track.gpx | grep '<time>' | head -n 1 | sed 's/<[^>]*>//g' | xargs)

if [ -n "$video_time_str" ] && [ -n "$gpx_time_str" ]; then
    video_sec=$(date -d "$video_time_str" +%s 2>/dev/null || echo "")
    gpx_sec=$(date -d "$gpx_time_str" +%s 2>/dev/null || echo "")

    if [ -n "$video_sec" ] && [ -n "$gpx_sec" ]; then
        offset=$(( video_sec - gpx_sec ))
        echo "Calculated gpx_offset: $offset seconds"
        echo "{\"gpx_offset\": $offset}" > var/dataset/video.json
    else
        echo "Failed to parse timestamps: video_time='$video_time_str', gpx_time='$gpx_time_str'"
    fi
else
    echo "Missing video creation_time or GPX start time"
fi

# (3) Transcode the file
echo "=== Transcoding the video ==="
time ffmpeg -i ./var/dataset/video.mp4 -vf "scale=-2:360,format=yuv420p" -c:v libx264 -crf 30 -preset veryfast -movflags +faststart -an var/dataset/video-360p.mp4

ls -lh var/dataset/*

# (4) Upload the results (target mp4 and video.json if it exists).
if [ -n "$DATASET_URL" ]; then
    echo "=== Uploading results ..."
    time aws s3 sync --no-progress var/dataset/ "$DATASET_URL/"
fi
