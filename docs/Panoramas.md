# Armchair Mapping with 360 Panoramas

This document describes the armchair mapping tool, which allows users to add trees to the map using 360-degree street-level imagery uploaded to Mapillary.

## Workflow Overview

Instead of physically visiting every tree with a GPS receiver, users can record 360 videos, process them, upload them to Mapillary, and then triangulate tree positions from their computer.

The mapping workflow follows these steps:

- mapping preparation: record 360 video and GPS tracks, process them into geotagged images, and upload them to Mapillary.
- background synchronization: the backend periodically pulls metadata for uploaded panoramas using the `mapillary-pull` command and caches them in the local database.
- workspace navigation: the user opens the armchair mapping page where Mapillary sequences are displayed as tracks on the map.
- ray casting: the user opens an image, aligns a tree in the center of the viewer, and casts a ray in that direction.
- triangulation: the user opens a different image from another angle, points to the same tree, and casts another ray. The system calculates the intersection of the rays and proposes a new tree location.
- creation: the user confirms the proposed location, enters the species and properties, and creates the tree on the map.

## Database Caching

To ensure fast rendering of tracks on the map and avoid querying the Mapillary API during map interaction, the backend maintains a local cache of panorama metadata.

The database schema includes a `mapillary_images` table with the following structure:

- id: unique Mapillary image identifier.
- sequence_id: identifier of the sequence/track.
- lat: latitude of the camera.
- lon: longitude of the camera.
- compass_angle: heading direction of the camera.
- captured_at: timestamp when the image was taken.

## Triangulation Mathematics

Triangulation uses 2D vector geometry to find the intersection of two or more rays cast from different camera positions.

Since mapping operations occur over small distances, we convert coordinates into a local flat-earth Cartesian projection (in meters) relative to the first camera position.

For each ray, we define:

- camera position: represented as a 2D coordinate point P = (x, y) in meters.
- bearing unit vector: represented as d = (sin(theta), cos(theta)) where theta is the compass angle of the ray in radians.

The intersection point I of two rays (PA + t _ dA) and (PB + s _ dB) is solved by finding the scale factor t using the following system of linear equations:

t _ sin(theta_A) - s _ sin(theta_B) = x_B - x_A
t _ cos(theta_A) - s _ cos(theta_B) = y_B - y_A

If the angle between the rays is too small, the triangulation may be imprecise, and the interface will warn the user. For three or more rays, a least-squares optimization is applied to find the point that minimizes the sum of squared perpendicular distances to all rays.

## Mapillary API Configuration

The integration requires configuring the public username and a client token for authentication with Mapillary.

- mapillary_username: this public username is stored in `config.toml` to filter and sync tracks.
- mapillary_client_token: this client token is stored securely as a file-based secret named `MAPILLARY_CLIENT_TOKEN` in the configured secrets directory (or as an environment variable for legacy support).

## Implementation Plan

- [x] database schema update: create the `mapillary_images` table and `mapillary_sequences` table in SQLite schema.
- [x] configuration registration: add `mapillary_username` to the config structures and `mapillary_client_token` to the `Secrets` struct in the backend.
- [x] synchronization command: implement a `mapillary-pull` command in the Rust backend to fetch sequence metadata and cache it locally.
- [x] backend API endpoints: expose endpoint `/v1/mapillary/geo.json` to serve cached data (images and tracks) as GeoJSON.
- [ ] panoramic layer display: render Mapillary tracks and image points as layers on the MapLibre map on the client.
- [ ] workspace page routing: set up a split-screen layout on a dedicated `/armchair` Svelte page.
- [ ] MapillaryJS integration: embed the interactive panoramic viewer inside the `/armchair` page.
- [ ] ray casting tool: add the visual ray line on MapLibre which updates dynamically with the active image coordinates and camera orientation.
- [ ] triangulation engine: implement the 2D multi-ray vector intersection calculations on the frontend.
- [ ] creation flow: add a button to pre-fill the tree creation form with the triangulated coordinates.
- [ ] validation and testing: write tests for triangulation math and verify everything with build and formatting.
