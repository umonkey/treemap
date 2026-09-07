# Street Panoramas

The platform supports 360° street-level panoramas to enable remote auditing, asset inventorying, and computer-vision pipelines directly from equirectangular video and GPS logs.

## Goals

Street panoramas provide spatial context for urban asset management, remote fieldwork, and automated feature extraction. The pipeline is designed to work with affordable consumer hardware rather than specialized, industrial-grade surveying equipment.

## Civic and Operational Value

For municipalities, environmental non-profits, and civic mapping communities, street panoramas transform field operations:

- Rapid city coverage: a single volunteer driving 20 to 40 km/h with an action camera can capture an entire neighborhood in an afternoon, replacing weeks of tree-by-tree foot surveys.
- Armchair mapping: community members who cannot conduct physical field surveys can participate from home, placing tree markers and cataloging species directly in a browser.
- Multi-year visual audit trail: re-surveying streets annually creates a historical record to evaluate canopy growth, pruning quality, storm damage, and tree mortality over time.
- Open GIS compatibility: spatial data and camera trajectories export directly to open formats (GeoJSON and standard WGS84) for immediate use in QGIS, ArcGIS, or OpenStreetMap without proprietary vendor lock-in.

## User Features

- Interactive 360 viewer: inspect street-level equirectangular imagery with smooth panning and zooming.
- Split-pane synchronization: a side-by-side viewer and map pane simultaneously display the panorama and camera position, keeping spatial orientation synchronized in real time.
- Remote auditing: inventory urban assets, inspect tree canopies, and verify ground features from desktop.
- Visual hints: display overlays for mapped objects, such as trees and adjacent sequence nodes.
- Trajectory reconstruction: reconstruct camera motion and positions to georeference every extracted frame.
- Sightline ray casting: project the current viewing direction onto the map to triangulate landmarks and verify positions.

## Tree Inventory Workflow

The panorama interface connects street imagery directly to the tree catalog:

- Sighting trees: navigating to any frame displays existing tree markers as overlays in the 360 view.
- Cross-frame triangulation: sighting the same tree trunk from two or more consecutive camera positions projects intersecting sightline rays onto the map, determining the exact trunk coordinates without standing under canopy obstructions.
- Direct attribute editing: clicking an existing tree overlay opens its profile to update species, trunk circumference, crown diameter, health status, or maintenance needs.
- New tree placement: clicking the ground level in the viewer or using ray intersections creates a new tree record at the projected coordinate.

## Equipment and Setup

Surveys require no expensive surveying rigs or calibrated vehicles:

- Cameras: any consumer 360 camera that exports equirectangular video (such as Insta360 X3 or X4, DJI Osmo 360, or GoPro Max).
- Mounting: a standard magnetic roof mount for vehicles, a seatpost mount for bicycles, or a handheld selfie stick for pedestrian alleys and parks.
- Location tracking: any smartphone running a free GPX recording app (such as GPS Logger or myTracks). No dedicated GNSS antenna is required.

## Operational Cost and Infrastructure

- On-demand compute: processing runs on AWS Batch using EC2 spot or on-demand instances that scale to zero when idle, avoiding ongoing server overhead.
- Predictable job costs: compute costs scale strictly with uploaded video duration, typically costing under two dollars per street sequence.
- Storage efficiency: raw videos can be archived or deleted after frame extraction, while lightweight extracted frames and metadata are stored in standard S3-compatible buckets.
- Cost accountability: the platform tracks ongoing resource expenses per panorama in the administration interface, reporting total accumulated processing time and compute cost (assuming standard hourly rates) alongside monthly S3 storage cost projections.

## Limitations

- Visual-only orientation: reconstruction relies exclusively on visual features across frames, requiring no compass or other IMU data.
- Absolute accuracy: reconstructions preserve precise internal geometric rigidity, but global coordinates depend on the input GPS track (typically within ±3 meters).
- Cloud processing: pipeline execution requires an AWS account configured with AWS Batch for background compute.

## Field Recording

Data collection requires only a consumer 360 camera and a smartphone:

- Video capture: record equirectangular video using the lowest available frame rate (for sharper individual frames) and the fastest shutter speed supported by ambient light (to prevent motion blur).
- GPS logging: record a separate GPX track with any GPS logging app (such as GPS Logger). Device clocks do not need to be synchronized.
- Loop closures in dense areas: when recording parks or dense courtyards, cross paths intentionally (using figure-8 trajectories or grid loops) to provide visual overlap that strengthens the 3D reconstruction.
- Featureless surfaces: avoid walking too close to monotonous walls, fences, or uniform surfaces, as the lack of visual keypoints can cause trajectory distortions.

## Video Conversion

Source footage must be exported as an equirectangular video file. Direction lock is unnecessary because camera heading is computed during reconstruction, but horizon leveling is recommended.

## Processing Pipeline

The ingestion and reconstruction workflow is fully automated:

- Upload: upload the video file (`.mp4`) and corresponding GPS log (`.gpx`). The processing job starts automatically.
- Notifications: the system sends an email notification upon job completion or failure.
- Failure recovery: if a run fails, the job can be restarted directly from the interface once the underlying issue is resolved.
- Sequence preview: upon completion, administrators can inspect the reconstructed sequence in a private preview before publishing it to the public map.
- Map fine-tuning: after publication, fine-tune alignment by sighting known reference landmarks (such as trees or building corners) to establish the best fit against the base map.

## Data Ownership and Governance

- Complete data sovereignty: all extracted imagery, trajectories, and point clouds reside in your own S3-compatible storage, ensuring complete data ownership and privacy control.
- Staged publication: sequences remain private until explicitly approved by administrators, allowing quality review before public access.
- Data export: administrators can export complete panorama sequence metadata and trajectories in JSON format for external GIS analysis or archiving.
