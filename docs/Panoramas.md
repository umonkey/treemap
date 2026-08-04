# Street Panoramas

The app has built in support for 360° street panoramas. This features processing of equirectangular videos with external GPX tracks, synchronizing them, extracting still frames at every 3 meters, reconstructing the scene using OpenSfM, and creating series of images to display in the app.

## Goals

Street panoramas are designed for high-performance automated data extraction, remote inventorying, and automated computer-vision pipelines, rather than being limited to visual navigation. They provide comprehensive spatial context for analyzing urban assets and mapping features efficiently from the desktop or field.

## User Features

- Interactive 360 viewer: allows users to explore street-level equirectangular imagery smoothly.
- Map layer navigation: enables switching between map views and panorama sequences seamlessly.
- Armchair mapping: supports remote auditing and data collection from recorded imagery.
- Panorama hints: guides users through sequences and highlights relevant features or adjacent nodes.
- Spatial sequence alignment: aligns video frames and GPS tracks for precise geospatial positioning.

## Field Recording

Recording imagery for the system involves capturing video and GPS tracks in the field:

- Video recording: record a video track with the highest available resolution and maximum available shutter speed to avoid motion blur. For armchair mapping, driving closer to the middle of the road works best; 30 fps at 60 km/h gives you a frame every 55 cm, so you do not need to drive very slowly.
- GPS track logging: record a separate GPX track using an application like GPS Logger (Android) or myTracks (iPhone).
- Clock synchronization: before recording, connect the camera app (e.g., DJI Mimo) to synchronize the camera clock with your phone. If the video and GPS track are out of sync, manual alignment using the application UI video sync feature will be required. Synchronizing clocks beforehand makes this alignment process significantly simpler.

## Video Conversion

The video file coming from the camera contains two video tracks with round videos (one for each lens) and a track with accelerometer data used for stabilizing the video later. You need to use DJI Studio to convert it.

Load all your recorded `.osv` files into DJI Studio, enable direction lock, and export as a panoramic video in the highest possible quality. After processing, you will have an MP4 file with a stabilized equirectangular video.

## Processing Overview

The workflow for adding new panoramas to the system is as follows:

- Upload: the user uploads an equirectangular 2:1 video file and a corresponding `.gpx` track.
- Synchronize: the user manually synchronizes the video with the GPS track using a map-based interface.
- Process: once synchronized, the data is queued for processing.
- Duration: processing normally takes 30 to 60 minutes per street.

## Infrastructure

We use AWS Batch to offload heavy processing from the main backend server. This ensures that the application remains responsive while video frames are being extracted and processed.

## Data Ownership

All images and processed data are stored on the user's S3 compatible buckets, ensuring full data ownership.

## Exporting Data

Admins can download all information on a panorama in `json` format using the export feature. There is no way to import it back just yet.
