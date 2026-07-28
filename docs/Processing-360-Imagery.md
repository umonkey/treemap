# Processing 360 Imagery

This document describes the process of recording a video track and making that recording available in the web application for [armchair mapping](https://wiki.openstreetmap.org/wiki/Armchair_mapping). It includes instructions for recording and converting the recorded files. This document focuses on using the DJI Osmo 360 camera, but should be applicable to other cameras, too.

The whole workflow looks like this:

1. Record video and GPS.
2. Convert to panoramic video (DJI Studio).
3. Upload to the application.
4. Synchronize and process internally.

## Recording the video

1. Record a video track with the highest available resolution and maximum available shutter speed to avoid motion blur. For armchair mapping, driving closer to the middle of the road works best; 30 fps at 60 km/h gives you a frame every 55 cm, so you do not need to drive very slowly.
2. Record a separate GPX track using an application like [GPS Logger](https://play.google.com/store/apps/details?id=eu.basicairdata.graziano.gpslogger) (Android) or [myTracks](https://apps.apple.com/us/app/mytracks-the-gps-logger/id358697908) (iPhone).
3. Before recording the video, make sure you connect the DJI Mimo app to your camera to synchronize the camera clock with your phone. Otherwise the GPS data might be unusable.

## Converting the video

The video file coming from the camera contains two video tracks with round videos (one for each lens) and a track with accelerometer data used for stabilizing the video later. You need to use DJI Studio to convert it.

Load all your recorded `.osv` files into DJI Studio, enable direction lock, and export as a panoramic video in the highest possible quality. After processing, you will have an MP4 file with a stabilized equirectangular video.

## Uploading to the Application

Once you have the equirectangular video and the corresponding GPX track, upload them to the application. The application handles synchronization, extraction of still frames, and scene reconstruction using OpenSfM internally.
