# Custom tiles

You can use a consumer grade drone to capture aerial imagery and create custom map tiles for better mapping.
(Using Litchi, or DJI waypoints, or just manually taking series of shots.)
This can give you higher resolution and more up-to-date maps than standard satellite imagery providers.
This document describes how to do that.

## 1. Capture imagery

Use a drone to capture overlapping images of the area you want to map.
Make sure to capture images with sufficient overlap (60-80%) to ensure good stitching later.

You can use [Litchi][1] and [ancient.land][2] to automate the process, or native waypoints in later DJI drones.
Or you can just manually fly the drone and take pictures, just make sure that when you take a picture, the camera looks straight down.

In the end, you get a ton of geotagged JPEG images.
Download them from the drone.

## 2. Stitch images into orthomosaic

This means combining the overlapping images into a single large image, called an orthomosaic.
This is best done with the ODM (OpenDroneMap) software, using Docker.
See the provided script [tools/drone-create-geotif.sh][4] for details.

In the end, if all is good, you get files `odm_orthophoto.tif` with the desired image, and `report.pdf` with some information about it.
You don't need the PDF report, just for curiosity.
The source JPEG files also will not be used anymore in this process.


## 3. Create map tiles

When you have a bunch of GeoTIFF files, you can convert them into map tiles using `gdal2tiles.py`, which is part of the GDAL library.
See the provided script [tools/drone-stitch.sh][5] for details.

In the end, you will get a folder named `tiles` full of small PNG files.
That's what you need in the next steps.


## 4. Publish your tiles

You can publish your tiles on a web server.
Any web server capable of serving static files will do.
Or you can use a premium service like [MapTiler Cloud][3] or Mapbox to host and serve your tiles.


[1]: https://flylitchi.com/
[2]: https://ancient.land/
[3]: https://cloud.maptiler.com/tiles/
[4]: ../tools/drone-create-geotif.sh
[5]: ../tools/drone-stitch.sh
