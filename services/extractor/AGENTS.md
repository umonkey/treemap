# Street Panorama Reconstruction

This service creates sequences of accurately geo-referenced 360° images from a source 360 video and a GPS track.


## Constraints

- The video comes from a consumer grade camera, like DJI Osmo 360, or Insta360.
- Two types of recordings are available: slow walking videos in dense environments, like (1) parks or courtyards, with dense canopies above head, and (2) fast driving videos, with the camera mounted on a car roof.
- Walking videos normally loop and have intersecting paths; driving videos are normally per-street, long and almost straight drives, with occasional gentle turns.
- The GPS data comes from a consumer grade device, like a mobile phone, recorded separately; data accuracy is 6 meters at best, 12 meters at worst.
- The GPS track might be much longer than the video track; one GPS track can be recorded for the whole driving session, spanning multiple streets and navigating between them, with multiple shorter video tracks recorded while driving a particular street.
- There is absolutely no metadata available from the video.  No GPS, no IMU. Extracted image EXIF timestamps default to 1970-01-01 and must NOT be relied upon for synchronization. Frame timeline is purely relative: `t_i = (index - 1) * FRAME_INTERVAL / fps`.
- There is no way to synchronize the video with the GPS track using any metadata, other than trajectory matching algorithms.
- Pipeline artifacts must be strictly non-destructive: `reconstruction.json` and `track.gpx` are strictly read-only. The alignment step emits an intermediate `var/dataset/trajectory.geojson` for inspection and downstream processing.
- Never snap or interpolate individual camera poses to noisy GPS points; the entire SfM reconstruction must be transformed as a single rigid Sim(3) block (scale, yaw, translation) to preserve ray collinearity for tree triangulation.
- There has to be one automated process to convert source `video.mpg` and `track.gpx` into a sequence of geo-referenced JPEG files enriched with pitch, roll and yaw data.


## Software Stack

- The services runs in a Docker container at AWS Batch.  The source files are read from S3, the results are uploaded to S3.  No other communication channels are available.
- OpenSfM: latest version from Mapillary.
- Python: 3.11


## Verification Commands

- Formatting: `uv run isort app/ tests/ && uv run black app/ tests/`
- Linter: `uv run flake8 app/ tests/`
- Type check: `uv run mypy app/`
- Tests: `uv run python -m unittest discover tests`
