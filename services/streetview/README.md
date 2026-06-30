# Street View Service

This service contains a container contains tools required to process an equirectangular 360 video and a `.gpx` file to a group of accurately geo-tagged `.jpg` images.  The idea is to make this container running on AWS Batch for extra performance and parallelization.  It can also be used locally as a pre-configured collection of the right tools (mainly during the testing phase).

## Performed tasks

1. Video transcoding.  For the GPS sync step we need a video file that can be quickly played on the web.
2. Extract geo-tagged jpeg files at certain intervals.
3. Process the input jpeg files with OpenSfM to restore accuracy.


## Usage Examples

### (1) Synchronize GPX track with the video

For this to work best you need to find the moment in the video where you move by a well known GCP (ground control point) at your normal/max speed, get the frame number and the coordinates of the GCP from OSM.  (Note that it won't work if you're standing still, e.g. on an intersection.)

To get the frame number, you can use the `bin/mpv-frames` like this:

```
bin/mpv-frames filename.mp4
```

Then run the app with the frame number and coordinates specified:

```
uv run python3 -m app synchronize --frame 1024 --lat 40.2037352 --lon 44.5074044 input.mp4 input.gpx
```

This command updates the video file `creation_time` meta with a timestamp that synchronzies it with the GPX track perfectly.

### (2) Extract the geotagged images

When you have your video synchronized with the GPX track, you can extract the frames:

```
mkdir -p frames
bin/extract input/video.mp4 input/track.gpx ./frames --distance 3.0
```


### (3) Process images with OpenSfM
