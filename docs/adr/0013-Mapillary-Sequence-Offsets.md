# ADR 0013: Mapillary Sequence Offsets

- Date: 2026-06-24
- Status: accepted

## Context

Mapillary imagery is often recorded using mobile devices or consumer cameras where GPS accuracy can be insufficient, especially in dense urban environments with high-rise buildings or narrow streets. This leads to panoramic sequences being displayed at a significant distance from their actual physical location on the map.

Since tree triangulation relies on the precise coordinate of the panorama, even a small GPS error (5-10 meters) can make it difficult to accurately place trees relative to other map features like buildings or sidewalks from OpenStreetMap.

## Decision

We will implement a manual coordinate offset capability for Mapillary sequences. This allow scientists to shift an entire sequence of images to match the visual reality of the map (OSM data).

Key implementation details:

- Database storage: add `lat_offset` and `lon_offset` columns to the `mapillary_sequences` table.
- Offset application: the offsets are applied at the API level when fetching image coordinates or calculating tree hints. This keeps the original raw Mapillary metadata intact while presenting the "corrected" position to the application.
- Admin UI: provide a numeric input in the sequence edit page to adjust these offsets, with real-time distance feedback in meters to make adjustments intuitive.
- Precision: use 64-bit floating point numbers for offsets to allow for extremely high precision adjustments (down to sub-meter levels).

## Consequences

- Improved accuracy: allow for precise tree triangulation even when source GPS data is poor.
- Better alignment: ensures street-level imagery perfectly matches the OpenStreetMap background.
- Armchair mapping: significantly improves the quality of armchair mapping by removing the friction of misaligned imagery.
- Data integrity: preserves original Mapillary metadata while layering corrections on top.
