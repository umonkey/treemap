# ADR 0023: Automated Panorama Trajectory Alignment

- Date: 2026-09-06
- Status: accepted

## Context

Processing 360 street view videos previously required an intermediate transcoding stage and manual video-to-GPS synchronization. Consumer 360 cameras (such as DJI Osmo 360 or Insta360) record high-resolution panoramic video without embedded GPS or IMU telemetry; extracted frames default to an epoch timestamp of 1970-01-01.

Previously, the ingestion pipeline operated in several phases:

- Transcoding: an AWS Batch worker running ffmpeg in `services/transcoder` generated low-bitrate web preview videos from 8K source footage.
- Manual synchronization: an administrator scrubbed video frames alongside a recorded GPX track in the web interface to manually synchronize video start offsets with GPS timestamps.
- Reconstruction with GPS: running OpenSfM using consumer GPS data as priors (`bundle_use_gps: yes`) to place camera poses.

This workflow presented major operational and quality issues:

- High resource consumption: transcoding 8K video files in AWS Batch consumed substantial compute time, memory, and high-throughput disk I/O for temporary preview assets.
- Process bottlenecks: requiring manual synchronization in the administrative interface halted ingestion and created human dependency.
- Distorted reconstruction: consumer GPS data carries 6 to 12 meters of drift, urban canyon multipath reflections, and sudden jitter. Using these noisy coordinates as bundle adjustment constraints deformed the photogrammetric reconstruction, created banana effects, and compromised the internal relative accuracy required for tree triangulation.

## Decision

We will fully automate panorama ingestion by eliminating intermediate video transcoding and manual synchronization, reconstructing scenes via pure photogrammetry, and automatically matching the reconstructed camera path to the GPS track via rigid trajectory alignment.

Key architectural and implementation changes:

- Dropped transcoder service: remove `services/transcoder`, AWS Batch transcoding job definitions, and database transcoding statuses (`NEEDS_TRANSCODING`, `NEEDS_TRANSCODING_FINISH`).
- Removed manual synchronization: remove `VideoSync`, `VideoPlayer`, and `TrackPreview` UI components, along with the `NEEDS_SYNC` state machine status. Uploaded panoramas queue directly for extraction and reconstruction.
- Pure photogrammetric reconstruction: configure OpenSfM without GPS priors (`bundle_use_gps: no`, `align_method: none`, `align_orientation_prior: none`). OpenSfM builds an internally rigid, undistorted 3D scene solely from visual feature matching and bundle adjustment.
- Automated trajectory alignment: implement `bin/align-trajectory` (`services/extractor/app/trajectory.py`) using a 2D Umeyama similarity transformation for coarse temporal offset search, followed by iteratively reweighted least squares (IRLS) with Cauchy robust loss to fit scale, horizontal yaw, and 2D translation against the GPX track.
- Rigid block transformation: transform the entire SfM camera graph as a single rigid Sim(3) block, preserving ray collinearity and angular relationships for tree triangulation.
- Non-destructive pipeline: `reconstruction.json` and `track.gpx` remain read-only; alignment outputs an intermediate `trajectory.geojson` before final metadata assembly.
- Ground control point offset calibration: retain manual sequence offsets (`lat_offset`, `lon_offset`) in the administrative interface to calibrate global shift against OpenStreetMap using interactive viewer ray casting against ground control points (GCPs).

## Consequences

- Fully automated pipeline: panoramas move from upload to published status without human intervention.
- Improved spatial precision: isolating OpenSfM from noisy GPS priors prevents geometric distortion, ensuring consistent depth and ray intersection accuracy during tree triangulation.
- Resource savings: eliminated AWS Batch transcoding compute jobs, high-throughput disk requirements, and temporary video storage.
- Codebase simplification: eliminated redundant video playback packages, frontend sync widgets, and backend transcoding dispatchers.
- Handling global GPS drift: because consumer GPS data carries global translation error, the automated alignment accurately resolves heading, scale, and relative shape, but inherits the GPS track's global shift. Administrators calibrate this remaining global offset by sighting identifiable ground control points (e.g., building corners) with viewer ray casting.
