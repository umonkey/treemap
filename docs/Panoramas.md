# Street Panoramas

The app has built in support for 360° street panoramas. This features processing of equirectangular videos with external GPX tracks, extracting still frames at regular intervals, reconstructing the 3D scene using OpenSfM, automatically aligning camera trajectories to the GPS track, and creating series of georeferenced images to display in the app.

## Goals

Street panoramas are designed for high-performance automated data extraction, remote inventorying, and automated computer-vision pipelines, rather than being limited to visual navigation. They provide comprehensive spatial context for analyzing urban assets and mapping features efficiently from the desktop or field.

## User Features

- Interactive 360 viewer: allows users to explore street-level equirectangular imagery smoothly.
- Map layer navigation: enables switching between map views and panorama sequences seamlessly.
- Armchair mapping: supports remote auditing and data collection from recorded imagery.
- Panorama hints: guides users through sequences and highlights relevant features or adjacent nodes.
- Spatial sequence alignment: aligns video frames and GPS tracks automatically for precise geospatial positioning.
- Ray casting sightlines: projects viewer viewing direction onto the map for landmark triangulation and coordinate verification.

## Field Recording

Recording imagery for the system involves capturing video and GPS tracks in the field:

- Video recording: record a video track with the highest available resolution and maximum available shutter speed to avoid motion blur. For armchair mapping, driving closer to the middle of the road works best; 30 fps at 60 km/h gives you a frame every 55 cm, so you do not need to drive very slowly.
- GPS track logging: record a separate GPX track using an application like GPS Logger (Android) or myTracks (iPhone).
- Clock synchronization: before recording, connect the camera app (e.g., DJI Mimo) to synchronize the camera clock with your phone. Automated trajectory alignment matches the motion profile of the video to the GPS track algorithmically, eliminating manual video-to-track synchronization in the user interface. Having accurate timestamps makes spatial correlation faster and more robust.

## Video Conversion

The video file coming from the camera contains two video tracks with round videos (one for each lens) and a track with accelerometer data used for stabilizing the video later. You need to use DJI Studio to convert it.

Load all your recorded `.osv` files into DJI Studio, enable direction lock, and export as a panoramic video in the highest possible quality. After processing, you will have an MP4 file with a stabilized equirectangular video.

## Processing Overview

The workflow for adding new panoramas to the system is fully automated:

- Upload: the user uploads an equirectangular 2:1 video file and a corresponding `.gpx` track.
- Automated processing: once files are uploaded, the dataset queues directly for background processing in AWS Batch without manual video synchronization or intermediate preview transcoding.
- Feature extraction and reconstruction: runs OpenSfM to reconstruct the local 3D scene from extracted frames using pure relative photogrammetry without GPS constraints.
- Trajectory alignment: the pipeline matches the reconstructed camera trajectory to the GPX track via rigid similarity transformation (scale, yaw, translation) using robust loss to filter GPS noise.
- Output generation: final georeferenced panorama frames and metadata are generated and uploaded to object storage.
- Duration: processing normally takes 30 to 60 minutes per street depending on video length.

## Positional Accuracy and OpenSfM

Consumer-grade GPS loggers (such as smartphone apps or action cameras) experience 6 to 12 meters of positional drift and urban multipath noise. Using raw GPS points directly during 3D reconstruction introduces severe distortions and bending (the banana effect).

To prevent geometric distortion, OpenSfM is configured to perform pure relative photogrammetry (`bundle_use_gps: no`). It builds an internally rigid, non-deformed local 3D reconstruction purely from visual features and camera bundle adjustment.

Once reconstruction is complete, the trajectory alignment tool (`bin/align-trajectory`) correlates the relative camera path with the timestamped GPX track. It uses a 2D Umeyama similarity transformation followed by iteratively reweighted least squares (IRLS) with Cauchy robust loss to determine time offset, scale, horizontal yaw, and translation. The entire camera graph is transformed as a single rigid Sim(3) block, preserving the ray collinearity and angular relationships necessary for accurate tree triangulation.

## Global Positioning and Ground Control Point Alignment

Although automated trajectory alignment resolves relative scale, heading, and geometry, it inherits any global translation offset present in the raw GPS track.

To align the finished sequence with the base map:

- Landmark selection: identify visible ground control points (GCPs) present in both the panorama imagery and OpenStreetMap, such as building corners or utility poles.
- Ray casting: in the administrative panorama preview, looking at a landmark in the 360 viewer projects a sightline ray onto the map. Sighting the same landmark from multiple camera positions creates intersecting rays indicating the actual ground position.
- Sequence offsets: administrators enter `lat_offset` and `lon_offset` values on the panorama edit page to translate the entire sequence to match OpenStreetMap data.

## Infrastructure

We use AWS Batch to offload heavy processing from the main backend server to EC2 instances equipped with high-throughput GP3 storage. The extractor container performs frame extraction, OpenSfM reconstruction, trajectory alignment, and result bundling in a single automated job without requiring intermediate transcoding services.

## Data Ownership

All images and processed data are stored on the user's S3 compatible buckets, ensuring full data ownership.

## Exporting Data

Admins can download all information on a panorama in `json` format using the export feature. There is no way to import it back just yet.
