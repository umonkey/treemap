# Pulling Data from OSM

The `treemap osm-pull` command synchronizes data from OSM using a two-phase process.

## Instances

The command iterates over enabled instances where `osm_pull` is enabled, ordered by id.

Each instance must have a complete bounding box (`bbox_w`, `bbox_s`, `bbox_e`, `bbox_n`). When the bbox is incomplete, the command logs a warning and skips the instance. The bot user id comes from `osm_bot_id`; when it is `NULL` or `0`, the command falls back to `bot_user_id` in `config.toml`. If the effective id is `0`, the instance is skipped with a warning.

## Overpass query

The query is built from the instance bbox, which is stored as `[west, south, east, north]`. Overpass expects `(south, west, north, east)`, so the values are reordered:

```text
[out:json];node[natural=tree](south,west,north,east);out meta;
```

The `out meta;` part is required because the merge logic needs the node version, timestamp, uid, and user name.

## Two-phase process

1. Retrieve current data from OSM. We use the Overpass API to fetch all tree nodes within the instance bounding box. These nodes are stored in the `osm_trees` table, which serves as a local cache of the OSM data and speeds up processing.
2. For each OSM node in the instance bbox, add a new tree to the local `trees` table or update an existing record with new data from OSM.

Remote changes are applied using the following algorithm:

1. We use the local tree's `osm_id` field to find corresponding trees.
2. If there is no local tree with that OSM id, we create a new one.
3. If there is a local tree, but the node was deleted on the OSM side, we mark the tree as gone on our side.
4. If the tree exists on both sides, and the remote version is newer, we update our local copy. We use the OSM node version number to track updates.
5. If the tree was edited locally since the last sync, only the coordinates are updated from OSM. Otherwise, all supported tags are updated.

## Cache behaviour

`osm_trees` is a global cache keyed by OSM node id and shared by all instances. Visibility updates are scoped to the current instance bbox, so pulling one instance does not mark another instance's cached nodes as invisible. Nodes outside the instance bbox are left untouched.

## Failure handling

Each instance is processed in its own database transaction. When an instance fails, the error is logged, the loop continues with the next instance, and the command exits with a non-zero status after processing all instances. Already committed instances are not rolled back.

## Scheduling

We pull updates every hour. The command is sequential and preserves the Overpass retry and backoff behaviour, so adding instances increases the total runtime.
