# Street Panoramas

The app has built in support for 360° street panoramas. This features processing of equirectangular videos with external GPX tracks, synchronizing them, extracting still frames at every 3 meters, reconstructing the scene using OpenSfM, and creating series of images to display in the app.

## Processing Overview

The workflow for adding new panoramas to the system is as follows:

- upload: the user uploads an equirectangular 2:1 video file and a corresponding `.gpx` track.
- synchronize: the user manually synchronizes the video with the GPS track using a map-based interface.
- process: once synchronized, the data is queued for processing.
- duration: processing normally takes 30 to 60 minutes per street.

## Infrastructure

We use AWS Batch to offload heavy processing from the main backend server. This ensures that the application remains responsive while video frames are being extracted and processed.

## Data Ownership

All images and processed data are stored on the user's S3 compatible buckets, ensuring full data ownership.
