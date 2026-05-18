# ADR 0010: Metric Update Tracking

- Date: 2026-05-18
- Status: accepted

## Context

The application needs to track when specific tree attributes (height, diameter, circumference), photos, and observations were last updated. This is crucial for field review workflows to identify not just where data is missing, but where it is outdated and requires re-measurement.

Currently, the `trees` table only has a general `updated_at` timestamp, which changes whenever any property is modified. While the `trees_props` table stores a history of attribute changes, querying it for a large number of trees (targeting 1 million) would involve complex joins or subqueries, leading to performance degradation.

## Decision

We will denormalize the tracking of these specific updates by adding five new timestamp columns to the `trees` table:

- `height_updated_at`
- `diameter_updated_at`
- `circumference_updated_at`
- `images_updated_at`
- `observations_updated_at`

These columns will be of type `INT` (Unix timestamp), non-nullable, with a default value of `0`. We will also add indexes for each of these columns to ensure efficient filtering.

## Rationale

- Performance: for a high volume of data (1 million trees), simple indexed columns allow for extremely fast filtering and sorting.
- Simplicity: the search logic remains straightforward, avoiding the need for complex SQL or high-memory application-side filtering.
- Data integrity: we will use targeted SQL updates to modify only these specific fields, ensuring that concurrent updates to other tree properties are not overwritten.

## Consequences

- Application logic: the backend must be updated to refresh these timestamps whenever their corresponding data changes.
- Schema growth: the `trees` table will grow by 40 bytes (5 \* 8-byte integers) per row. This is a negligible trade-off for the performance gains.
- Initial migration: a one-time migration is required to backfill these columns from existing `trees_props`, `trees_images`, and `observations` data.
