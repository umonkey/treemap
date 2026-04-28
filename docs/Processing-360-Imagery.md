# Processing 360 Imagery

This document describes the process of recording a video track and making that recording available in the web application for [armchair mapping](https://wiki.openstreetmap.org/wiki/Armchair_mapping). It includes theory and step by step instructions for processing the recorded files. This document focuses on using the DJI Osmo 360 camera, but should be applicable to other cameras, too.

The whole workflow looks like this:

1. Record the video and GPS data.
2. Convert the raw image to a common format.
3. Extract images from the video into series of images at certain distance.
4. Clean up the images.
5. Upload results to Mapillary.

## Recording the video

1. You need to record a video track with highest available resolution, maximum available shutter speed (to avoid motion blur). For armchair mapping, driving closer to the middle of the road works best; 30 fps at 60 km/h gives you a frame every 55 cm, so you don't need to drive very slow.
2. You need to record a separate GPX track, using an application like [GPS Logger](https://play.google.com/store/apps/details?id=eu.basicairdata.graziano.gpslogger) (Android) or [myTracks](https://apps.apple.com/us/app/mytracks-the-gps-logger/id358697908) (iPhone).
3. Before recording the video, make sure you connect the DJI Mimo app to your camera, to synchronize camera's clock with your phone. Otherwise the GPS data might be unusable.

## Converting the video

The video file coming from the camera contains two video tracks with round videos (one for each lens, unusable in the raw form) and a track with the accelerometer data used for stabilizin the video later. You need to use DJI Studio to convert it.

Load all your recorded .osv files into DJI Studio, enable "direction lock" and export as "panoramic video" in highest possible quality. Few hours later, you will have an MP4 file with an requirectangular video, stabilized.

## Extracting the images

Use the [tm-geotag](https://github.com/umonkey/mapping-tools) tool to extract geo-tagged images from the video at certain distances, e.g. 3 meters. This will convert your video to a folder full of geotagged JPEG images which could then be used with any 360-capable software or platform, e.g. Mapillary.

## Cleaning up

Use the [Photini](https://photini.readthedocs.io/en/latest/) tool to clean up the images:

1. Remove unwanted images, e.g. those taken before you went on the right street, loops, whatever feels not needed.
2. Correct the coordinates. The GPS can drift, and the OSM layer is not always accurate, so your images can get significantly off track. Move your images to where they belong.

## Uploading to Mapillary

It is recommended to use `mapillary_tools` to upload the images from the command line. The tool tends to split the dataset into multiple tracks according to distance, time between images, total files etc. To avoid that, use a set of command line options and environment variables:

```bash
export MAPILLARY_TOOLS_MAX_SEQUENCE_PIXELS=100000000000
export MAPILLARY_TOOLS_MAX_SEQUENCE_COUNT=99999

mapillary_tools process_and_upload ./dst \
  --cutoff_distance 999999 \
  --cutoff_time 999999 \
  --duplicate_distance 0
```
