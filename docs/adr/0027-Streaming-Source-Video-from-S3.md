# ADR 0027: Stream Source Video from S3 During Frame Extraction

- Date: 2026-09-16
- Status: accepted

## Context

Source 360 videos can be up to roughly 70 GB. The extractor runs in an AWS Batch container whose filesystem provides about 100 GB in total, shared with the operating system, so downloading the full source video to local disk is unsafe and can exhaust the volume before any frames are written.

Previously, `bin/extract-frames` downloaded the source object to `$DATASET/video.mp4`, extracted frames at fixed spatial intervals, and then deleted the file to reclaim space. The pipeline only reads frames at those fixed intervals and never needs the whole file at once.

Storage is DigitalOcean Spaces, an S3-compatible object store. The source object is private, so direct HTTP access is not possible and a presigned URL is required.

## Decision

We stream the source video directly from S3 over the network instead of downloading it. `bin/extract-frames` generates a presigned HTTPS URL for `$DATASET_URL/video.mp4` and passes it to PyAV, which reads through FFmpeg using HTTP Range requests for seeking.

Key details:

- Presigning: the `aws s3 presign --endpoint-url="$AWS_ENDPOINT_URL" --expires-in "${VIDEO_PRESIGN_TTL:-43200}" "${DATASET_URL%/}/video.mp4"` command produces the URL.
- Fallback: a local `$DATASET/video.mp4`, when present, is used as an offline/debug source and no presigned URL is generated.
- No local copy: the extractor never writes the source video to disk and never deletes one.
- Upload hygiene: `bin/upload-dataset` excludes `video.mp4`, so a local debug source is never re-uploaded to the dataset bucket.
- PyAV options: for `http://` and `https://` sources, `av.open` receives FFmpeg reconnection options (`reconnect`, `reconnect_streamed`, `reconnect_delay_max`) so transient network drops do not abort extraction.
- Log hygiene: presigned signatures are stripped before logging, so the URL secret never reaches the logs.
- No new Python dependencies: presigning uses the AWS CLI already present in the image, and streaming uses PyAV's existing FFmpeg HTTP stack.

Alternatives considered and rejected:

- FUSE mount (`s3fs` or `mountpoint-s3`): requires a privileged container and `/dev/fuse`, and adds heavier operational setup for no functional gain.
- Custom boto3 seekable file object with a bounded chunk cache: adds a boto3 dependency plus custom seek and cache code, and depends on PyAV Python-IO seekability. Kept as a fallback if HTTP seeking misbehaves in production.
- Byte-range fetching of only the needed GOPs: requires demux and index mapping and is far more complex for marginal benefit over FFmpeg's own range seeking.

## Consequences

- Removes the 70 GB disk requirement, so extraction no longer competes with the operating system for space.
- Network transfer is roughly equal to a full download because decoding is largely sequential, though seeking adds some overhead.
- The run depends on network availability and on the presigned URL remaining valid. The default TTL is 12 hours and can be overridden with `VIDEO_PRESIGN_TTL`; SigV4 presigned URLs expire after at most 604800 seconds.
- A crash mid-extraction re-streams from the start of the video, mitigated by the existing per-frame resume logic that skips already written frames.
- Presigned URLs are secrets and must not be logged.
