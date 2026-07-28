# Panoramic video transcoder

This folder contains the Docker container image that is used to transcode (downsample) source 8K equirectangular videos into 360p for quicker web playback.  This is an intermediate step required for manual MP4-vs-GPX synchronization.

The workflow is the following:

1. Download the source `.mp4` file from S3.
2. Transcode using `ffmpeg`.
3. Upload the results to S3.

The container normally runs in AWS Batch, on on-demand instances.

The configuration is passed via environment variables.
