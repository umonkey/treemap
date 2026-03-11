# Armchair Mapping Feature

We need to integrate Mapillary to implement [armchair mapping](https://wiki.openstreetmap.org/wiki/Armchair_mapping).  The idea is that we take 360 imagery, upload it to Mapillary, then use a specialized UI to quickly triangulate and put new trees on the map.  This document contains the implementation plan.  It is temporary and will be removed as soon as the feature is implemented.


## Workflow Details

1. The user opens a specialized page, e.g. /armchair
2. The map loads with Mapillary tracks containing 360 imagery from a certain user.
3. The user clicks a 360 image, the image opens on a dedicated pane.
4. The viewing direction is displayed on the map as a 10 m long line.
5. The user centers on a tree and clicks a button, the line is remembered.
6. The user adds as many lines as they can see on the image.
7. The user switches to the next image and does the same.
8. The user adds trees to line intersections.  Effectively performs manual triangulation.
9. For each triangulated tree, remember: coordinates, Mapillary image id, direction settings.  Let the user guess species.


## What needs to be done

- [ ] Add a config option for the Mapillary API key.
- [ ] Add a config option for the Mapillary username, to filter tracks.
- [ ] Understand data format coming from Mapillary.
- [ ] Add a separate endpoint to retrieve Mapillary data points for adding to the map.
- [ ] Embed Mapillary 360 viewer as an overlay.
- [ ] Add "mapillary image id" to the trees.


## Future Enhancements

- [ ] Detect intersection of lines to auto-add trees.
