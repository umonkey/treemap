# Duplicate Tree Resolution

This document describes how the automated duplicate tree resolution works in the application.

## Overview

The resolution process identifies trees at the same geographical location (rounded to 7 decimal places) and merges them into a single record to maintain data integrity and prevent UI clutter.

## Merging Logic

The process follows these rules when resolving a group of duplicate trees:

- Main tree selection: the tree with the lowest ID (earliest created) is chosen as the primary record. All other trees in the group are considered secondary.
- Scalar property merging: for properties like height, circumference, and diameter, the value from the record with the latest update timestamp for that specific field is used.
- State exclusion: when merging the tree state, "gone" is ignored. This prevents the system from marking a physical tree as missing just because a duplicate record was deleted on the OSM side.
- Species exclusion: taxonomic names that are "Unknown" or contain "unknown" (case-insensitive) are ignored if a more specific name is available in the group.
- Text property merging: for notes and addresses, the latest non-empty value is preferred.
- Metadata merging: the `images_updated_at` and `observations_updated_at` timestamps are set to the maximum value found in the group.
- Thumbnail selection: if the main tree lacks a thumbnail, it inherits the thumbnail from the secondary record with the latest image update.

## Associated Data Migration

All data linked to secondary trees is reassigned to the main tree:

- Photos: all records in the `trees_images` table are moved to the main tree.
- Comments: all tree comments are moved, and the main tree's `comment_count` is updated.
- Likes: all likes are moved. The system handles potential conflicts where multiple users liked different duplicates of the same tree.
- Observations: all historical observations are migrated.
- Property history: the full audit trail from the `trees_props` table is preserved and linked to the main tree.

## Audit Trail

Secondary trees are not deleted from the database. Instead:

- State: set to "replaced".
- `replaced_by`: updated to point to the main tree ID.
- `osm_id`: the original OSM ID is preserved on the replaced record. This allows the OSM sync handler to identify which nodes need to be removed from OpenStreetMap.

## OSM Node Synchronization

Local merging uses a "lowest ID first" heuristic, which may not always match decisions made by humans directly on OpenStreetMap. For example, a user might delete node A (our main tree) as a duplicate of node B (one of our secondary trees).

To resolve this, the system provides a remapping mechanism:

- Identification: the system finds active local trees whose linked OSM nodes have been deleted (marked as `visible = 0` in the cache).
- Remapping: for each mismatched tree, it checks its "replaced" duplicates. If a duplicate is linked to an OSM node that is still alive, the system swaps the `osm_id` values between the main tree and the duplicate.
- Outcome: the active local tree is re-linked to the surviving OSM node, ensuring continuity while preserving the link to the deleted node in the audit record.

## CLI Usage

To run the duplicate resolution, use the following command:

```bash
treemap merge-duplicates [limit]
```

The limit argument is optional and defaults to 10 duplicate groups per run.

To fix OSM link mismatches after an OSM sync:

```bash
treemap osm-remap-duplicates
```

## Roadmap

- Remove leftover duplicates on OSM side: execute deletion requests via `osm-push-delete` to clean up the OpenStreetMap database.
- Automated resolution: integrate remapping and merging into the regular background maintenance tasks.
