# ADR 0026: Panorama Geometry Precomputation

- Date: 2026-09-12
- Status: accepted

## Context

The bounding box endpoint returned one full-track LineString per panorama. The geometry was built from the `points_json` column at response time, and the stored `lat_offset` and `lon_offset` were added to every coordinate while serializing the response. This had two limitations.

Panorama tracks are frequently trimmed by hiding individual images. A hidden image in the middle of a track should break the rendered line, so the visible portions must render as independent sections. A single LineString cannot express a track with gaps.

Applying offsets at the API level also duplicated offset arithmetic in every response and forced the API to read the offsets on each request. Precomputing the geometry moves this work to a single write path and keeps the response handler focused on serialization.

## Decision

We precompute the panorama line geometry as an array of sections and persist it in the `points_json` column. Each section is a maximal run of visible images with at least two points, and hidden images split runs. Sections with fewer than two points are dropped.

The `lat_offset` and `lon_offset` values are baked into the stored coordinates at generation time. This supersedes the offsets-applied-at-the-API-level clause of ADR 0013, which is replaced by the manual offset calibration workflow retained in ADR 0023.

We regenerate the geometry asynchronously whenever the inputs change. Hiding or showing an image and changing the panorama offset both enqueue an `UpdatePanoramaStats` queue command. The queue consumer reloads the panorama and its ordered images, recomputes the sections and bounds, and writes only the statistics columns. The row is persisted before the event is enqueued, so the consumer always reads the fresh status and file size.

The bounding box endpoint emits one MultiLineString feature per panorama. Coordinates are an array of sections, each section an array of `[lng, lat]` positions in RFC 7946 order.

Bounds are computed from visible images only, with offsets applied. When a panorama has no visible images, all four bounds and `points_json` are null. When it has visible images but no section of two or more points, only `points_json` is null.

Legacy rows store the old depth-2 coordinate array without offsets. The frontend normalizes depth-2 coordinates into a single section when loading, and a one-time `refresh-panorama-stats` CLI command enqueues a refresh for every panorama.

## Consequences

- Simpler responses: the bounding box endpoint no longer applies per-coordinate offset math, and the single-panorama endpoint keeps its LineString-per-section behavior.

- Hidden images render correctly: gaps appear as separate sections in a single MultiLineString feature.

- Cacheability: the stored geometry is a render-ready artifact, at the cost of a queue round trip before changes appear on the map.

- Consistency risk: the geometry is only as fresh as the last refresh, so the queue consumer must run for hide/show and offset changes to take effect.

- Migration: the legacy format remains readable until the backfill command runs, after which all rows use the sectioned format.
