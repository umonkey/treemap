#!/bin/bash
# The main transcoder script.  Does the following:
# (1) Download the original mp4 file
# (2) Optionally calculate gpx offset if track.gpx exists and upload video.json
# (3) Transcode to 360p
# (4) Upload the target mp4 file

set -e

if [ -z "$DATASET_URL" ]; then
    echo "Usage: DATASET_URL=s3://bucket/id ./convert.sh"
    exit 1
fi

# (1) Download the original mp4 file.
time aws s3 cp --no-progress "$DATASET_URL/video.mp4" ./source.mp4

# (2) Attempt to download track.gpx and calculate offset if it exists
if aws s3 cp --no-progress "$DATASET_URL/track.gpx" ./track.gpx 2>/dev/null; then
    echo "Found track.gpx, calculating gpx_offset..."
    video_time_str=$(ffprobe -v quiet -select_streams v:0 -show_entries stream_tags=creation_time -of default=noprint_wrappers=1:nokey=1 source.mp4)
    gpx_time_str=$(grep -om 1 '<time>[^<]*</time>' ./track.gpx | head -n 1 | sed 's/<[^>]*>//g')

    if [ -n "$video_time_str" ] && [ -n "$gpx_time_str" ]; then
        video_sec=$(date -d "$video_time_str" +%s 2>/dev/null || echo "")
        gpx_sec=$(date -d "$gpx_time_str" +%s 2>/dev/null || echo "")

        if [ -n "$video_sec" ] && [ -n "$gpx_sec" ]; then
            offset=$(( video_sec - gpx_sec ))
            echo "Calculated gpx_offset: $offset seconds"
            echo "{\"gpx_offset\": $offset}" > video.json
            aws s3 cp --no-progress video.json "$DATASET_URL/video.json"
        else
            echo "Failed to parse timestamps: video_time='$video_time_str', gpx_time='$gpx_time_str'"
        fi
    else
        echo "Missing video creation_time or GPX start time"
    fi
else
    echo "track.gpx not found, skipping offset calculation"
fi

# (3) Transcode the file
time ffmpeg -i ./source.mp4 -vf "scale=-2:360,format=yuv420p" -c:v libx264 -crf 30 -preset veryfast -movflags +faststart -an target.mp4
ls -lh target.mp4

# (4) Upload the results.
time aws s3 cp --no-progress target.mp4 "$DATASET_URL/video-360p.mp4"
