This page collects findings on how to integrate with OSM.
We currently publish our data as user [YerevanTreeMaps](https://www.openstreetmap.org/user/YerevanTreeMap).

## Current status

- [x] Import new trees from OSM.
- [x] Import tree updates from OSM.
- [x] Export new trees to OSM.
- [ ] Export changes to OSM.

## Workflow

The integration consists of two separate steps: pulling and pushing. While they use different commands, they are not independent:

- Pulling: the `treemap osm-pull` command should be run regularly to keep the local `osm_trees` cache up to date.
- Pushing: the `treemap osm-push` and `treemap osm-push-changes` commands rely on the `osm_trees` cache to correctly identify changes and avoid duplicates. Therefore, you must pull data before pushing any changes.

We pull updates every hour. We don't currently push any updates to OSM as that requires negotiation with the Data Working Group.

## OSM basics

In OSM, single trees are nodes with the following tags of interest:

- `natural=tree`: defines a single tree.
- `leaf_type`: can be mapped from species.
- `genus`: holds the first part of the latin scientific name, like _Quercus_ or _Fraxinus_. There are also localized versions like [genus:en](https://wiki.openstreetmap.org/wiki/Key:genus:en).
- `species`: the full latin name, like _Quercus robur_ or _Tilia cordata_. If this is set, then `genus` is not needed.
- `species:wikidata`: a code like _Q158746_ which has detailed information on the [Wikidata site](https://www.wikidata.org/wiki/Q158746), including names in many different languages and photos. This could be used for species suggestions.

## Pulling data from OSM

The `treemap osm-pull` command synchronizes data from OSM using a two-phase process.

1. Retrieve current data from OSM. We use Overpass API to fetch all tree nodes within the configured bounding box. These nodes are stored in the `osm_trees` table, which serves as a local cache of the OSM data and speeds up processing.
2. For each OSM node received, add a new tree to the local `trees` table or update an existing record with new data from OSM.

Remote changes are added using the following algorithm.

1. We use the local tree's `osm_id` field to find corresponding trees.
2. If there is no local tree with that OSM id, we create a new one.
3. If there is a local tree, but we see that the node was deleted on the OSM side, we mark the tree as gone on our side.
4. If the tree exists on both sides, and the remote version is newer, we update our local copy. We use OSM node version number to track updates.

## Pushing data to OSM

New trees are pushed to OSM using the `treemap osm-push` command. We look in the `trees` table to find trees that don't have an `osm_id`, create a new changeset and send 100 of those trees to OSM using the API. We then set the new received `osm_id` in the `trees` table. This is how we push new trees to OSM. This process normally runs every hour.

We don't currently push updates to existing trees.

## Configuration

To successfully pull data from OSM, the following options need to be set in `config.toml`:

- `bot_user_id`: the id of the local user account who owns the local changes, something like "OSM Bot".
- `overpass_query`: the query to retrieve the data, make sure you have proper boundaries set.

To push new trees to OSM, the following additional options are required:

- `osm_client_id`: the application id, get it [here](https://www.openstreetmap.org/oauth2/applications).
- `osm_hashtag`: this identifies your changesets, e.g. "YerevanTreeMap".
- `osm_activity`: a link to the OSM Wiki page describing the organized editing activity.

## Source Code Links

- Pulling data from OSM is in `backend/src/services/osm_reader/mod.rs`
- Pushing data to OSM is in `backend/src/services/osm_writer/mod.rs`
