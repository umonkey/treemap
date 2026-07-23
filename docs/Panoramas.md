# Street Panoramas

The app has built in support for 360° street panoramas.  This features processing of equirectangular videos with external GPX tracks, synchronizing them, extracting still frames at every 3 meters, reconstructing the scene using OpenSfM, and creating series of images to display in the app.

## Processing Overview

The user must provide an equirectangular 2:1 video, and a separate `.gpx` file.  The rest is handled by the application.  The following steps are taken:

1. Downsample the video to 360p for playing in the web.  This is needed to manually synchronize the recording with the GPS track.  We use AWS Batch for this.


### Downsampling the Video
