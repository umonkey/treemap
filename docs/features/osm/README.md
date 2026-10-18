# OpenStreetMap Integration

This page collects findings on how to integrate with OSM.
We currently publish our data as user [YerevanTreeMap](https://www.openstreetmap.org/user/YerevanTreeMap).

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

The `treemap osm-pull` command synchronizes data from OSM using a two-phase process. Each instance builds its own Overpass query from its bounding box, and only enabled instances with OSM pull turned on are processed.

See `pull/README.md` for the detailed algorithm, cache behaviour, and failure handling.

## Pushing data to OSM

New trees are pushed to OSM using the `treemap osm-push` command. We look in the `trees` table to find trees that don't have an `osm_id`, create a new changeset and send 100 of those trees to OSM using the API. We then set the new received `osm_id` in the `trees` table. This is how we push new trees to OSM. This process normally runs every hour.

We don't currently push updates to existing trees.

## Configuration

Pull settings are configured per instance in the `instances` table, either through the `/tools/configure` page or directly with SQL:

- `osm_pull`: enables pulling for the instance. Pull is disabled when the value is `NULL` or `0`.
- `osm_bot_id`: the local user id used for changes pulled from OSM. When `NULL`, the command falls back to `bot_user_id` in `config.toml`.
- `bbox_w`, `bbox_s`, `bbox_e`, `bbox_n`: the bounding box that defines the instance area. Pull is skipped with a warning when the bbox is incomplete.

The `overpass_endpoint` option remains in `config.toml` and is shared by all instances.

To push new trees to OSM, the following additional options are required in `config.toml`:

- `osm_client_id`: the application id, get it [here](https://www.openstreetmap.org/oauth2/applications).
- `osm_hashtag`: this identifies your changesets, e.g. "YerevanTreeMap".
- `osm_activity`: a link to the OSM Wiki page describing the organized editing activity.
