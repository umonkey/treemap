# ADR 0032: Append-only Tree Event Journal

- Date: 2026-11-08
- Status: proposed

This is the first ADR in this repository with a `proposed` status. It records a direction that is not implemented, so it should be accepted or superseded before the work starts.

## Context

Tree attribute history lives in `trees_props`. Every row is a `(tree_id, name, value, added_at, added_by)` tuple, and readers aggregate the latest row per name. The tree history endpoint, the edit heatmap, and the statistics queries in `docs/Statistics.md` all read it, and the per-metric timestamp columns from ADR 0010 were backfilled from it.

The table works as a display log but not as a reliable record of change. It is written beside the `trees` row rather than derived from it, and the two writes are independent.

- atomicity: `TreeRepository` updates `trees` and then inserts zero or more `trees_props` rows as separate statements. A failure between them leaves state and history inconsistent. Tree creation logs a partial baseline because it diffs against `Tree::default`, which omits fields whose default already matches, including `state`.
- typing: `value` is `TEXT NOT NULL`. Numbers are stored as text, a cleared field becomes an empty string, and there is no `old` value. Readers must re-parse values per field name.
- identity: `name` is free text. `location` packs two coordinates into one string and `merged_from` is a local convention, so the set of fields is implicit and typo-prone.
- immutability: history is rewritten. The tree merger changes the `tree_id` of existing rows, and migration `008-tree-state-alive.sql` overwrote historical `sick`, `deformed`, and `healthy` values in place.
- ordering: `added_at` has second precision, `find_by_tree` orders by it alone, and the row id is a snowflake, so two edits in one second have no defined order.
- concurrency: updates are read-modify-write with no revision guard, so a concurrent edit can be lost while the log records only the survivor.
- reads: `GET /v1/trees/{id}/history` returns the entire history for a tree and the frontend filters by field client-side. There is no index on `added_at`, although the heatmap and statistics scan by it.

The goal is a complete and trustworthy change history, a one-line-per-property history view, and a foundation that makes reverts possible later without implementing them now. The tree read model must stay a single indexed row per tree because spatial queries and filters target about one million trees (ADR 0010).

## Decision

Adopt an append-only event journal as the authoritative record of tree mutations, and keep the `trees` row as a materialized projection that is written in the same transaction as the event that justifies it.

### Event envelope

Use one generic table, `events`, keyed by a stream rather than a table per entity:

- `id`: snowflake, a global order.
- `stream_kind`: `tree`, and later other entity kinds.
- `stream_id`: the entity id as text.
- `stream_version`: a per-stream sequence used for optimistic concurrency. It is null for imported legacy history, which cannot participate in concurrency.
- `occurred_at`, `actor_id`, `actor_kind` (`user`, `bot`, `system`, `osm`), `kind` (`baseline`, `created`, `updated`, `confirmed`, `merged`, `replaced`, `removed`, `imported`), and `source` (`web`, `osm_pull`, `osm_push`, `import`, `merge`, `cli`, `backfill`).
- `correlation_id`: groups events emitted by one request or job across streams.
- `changes`: a JSON array of `{ field, old, new }` objects.
- `schema_version`, and an optional `external_ref` used to deduplicate retried writes.

One event represents one logical action, not one database transaction and not one field. A request that changes height, species, and state produces one `updated` event with three change objects. A confirmation of an unchanged value produces a `confirmed` event whose change object has `old` equal to `new`, so the existing confirmation workflow keeps its history line. Multi-stream actions, such as a merge, write one event per stream sharing a `correlation_id`.

JSON is used for `changes` because it keeps the event atomic, preserves value types (null versus zero versus text), and avoids a second table on the write path. A view expands it for the one-line-per-property history view:

```sql
CREATE VIEW tree_event_changes AS
SELECT e.id AS event_id,
       e.stream_id AS tree_id,
       e.occurred_at,
       e.actor_id,
       e.actor_kind,
       e.kind,
       e.source,
       json_extract(j.value, '$.field') AS field,
       json_extract(j.value, '$.old') AS old_value,
       json_extract(j.value, '$.new') AS new_value,
       j.key AS change_index
FROM events e, json_each(e.changes) j
WHERE e.stream_kind = 'tree';
```

The history endpoint filters and paginates by change, using `(occurred_at, event_id, change_index)` as the cursor, so a page always has the requested number of lines. It no longer loads the whole tree history.

### Projection and concurrency

Add `trees.revision INT NOT NULL DEFAULT 1`. Every write goes through one repository method that, in one transaction, reads the old row, computes the change set, updates `trees` with `WHERE id = ? AND revision = ?`, inserts the event with the new revision, and commits. A zero-row update returns a conflict instead of silently losing an edit.

The change set is produced by one `diff` function that destructures every writable field without a `..` rest pattern, so adding a field to `Tree` fails to compile until the diff covers it. All writers pass an explicit actor kind and source. OSM sync, the merger, and address backfills stop appearing as ordinary user edits. The per-metric timestamp columns from ADR 0010 are filled from the same field-to-column mapping instead of scattered call sites.

Append-only is enforced in the database with `BEFORE UPDATE` and `BEFORE DELETE` triggers that abort on `events`. The `trees` projection remains the only operational read model.

### Cutover

The schema migration is additive: it creates `events`, adds `trees.revision`, and creates the indexes and view. An idempotent CLI backfill then inserts one `baseline` event per existing tree with the current row snapshot, so replay has a starting point. It converts existing `trees_props` rows into `imported` events with `source = 'legacy'`, no `old` value, and no stream version, for display only. `trees_props` stays read-only for one release and is then dropped.

A `verify-tree-events` CLI folds events from the latest baseline and compares each field with `trees`, reporting drift per tree. It runs against the seed database in CI and periodically in production. A `rebuild-tree` command is optional and admin-only.

### Deferred

- Revert and time-travel endpoints and their conflict UX. Storing `old` in every change is the only preparation required. A revert later writes a new compensating event guarded by the revision.
- Event-sourcing any entity other than trees. Alerts, water sources, users, files, comments, and observations keep their current models. Observations keep the snapshot revisioning from ADR 0006.
- Replay as the source of truth at startup. The projection is persistent and the journal is verified, not rebuilt on every boot.

## Rejected alternatives

- Harden `trees_props`: add `old_value`, a type tag, a field enum, a foreign key, an index, and transactions. This fixes typing and atomicity but keeps one row per field with no action grouping, no baseline, no rebuild proof, and no place for actor source.
- Full event sourcing: rebuild every aggregate from events and treat the log as the only truth for all entities. The projection is still required for spatial and filtered tree queries, so the extra ceremony and migration risk buy nothing at this scale.
- One event table per entity: typed columns and foreign keys per entity, at the cost of duplicated write and read code, a `UNION` for a global activity feed, cross-table ordering, and duplicated retention policy. The generic stream table supports a new entity with an enum variant and no migration.
- Full-row snapshot per revision, as observations do: simple time travel and revert, but it duplicates long `notes`, grows faster, and loses field-level attribution unless changes are diffed anyway.
- A normalized child table of changes from day one: indexable per field, but it doubles the write path and the consistency surface. It can be added later without changing the event envelope if field-level analytics become hot.

## Consequences

- Completeness: a tree change and its history are committed together or not at all, and the creation baseline includes every field.
- Concurrency: a stale write fails with a conflict instead of overwriting another user's edit.
- Trust: heatmap, per-user statistics, and the ADR 0010 timestamp columns can be derived from one action record instead of three independent conventions.
- Preparedness: storing `old` values and a baseline means a revert endpoint can be added later without a migration.
- Drift detection: the verifier catches any code path that writes `trees` outside the event path.
- Write cost: each edit adds one transaction and one JSON encode. SQLite is shared with the chatbot, so transactions must stay short and the existing WAL and busy timeout settings must be preserved. A deferred `libsql` transaction that reads then writes can return `SQLITE_BUSY_SNAPSHOT`, so the write path needs an immediate transaction or a bounded retry.
- Generic envelope: the database no longer enforces per-entity foreign keys or column types. Type safety moves to the Rust event payload enum, and the verifier checks for orphaned streams.
- Retention: old `notes` and `address` values live indefinitely, so account deletion and redaction need an explicit policy.
- History before the cutover stays lossy. Migration 008 already destroyed some values, and legacy rows have no `old` value, so no replay can recover them.
- Scope: likes, view counters, files, and observations are not event-sourced. The events table must not become a log for high-volume telemetry or binary data.
- No revert UI, no time travel, and no cross-entity activity feed are delivered by this decision.

## Revisit trigger

This ADR records a direction, not scheduled work. Revisit and either accept or supersede it when any of the following happens:

- revert or time travel becomes a product requirement.
- a second entity needs a change history, which is the point to confirm or revise the generic stream envelope.
- `trees_props` causes a concrete data-loss or inconsistency incident.
- edit volume or the `events` table size makes the view or the verifier too slow.
- the deployment becomes multi-instance, which decides whether `instance_id` belongs in the envelope before the table is large.

Until then, `trees_props` receives only correctness fixes and no expansion.

## Appendix: schema sketch

```sql
CREATE TABLE events (
  id             INTEGER PRIMARY KEY,
  stream_kind    TEXT NOT NULL,
  stream_id      TEXT NOT NULL,
  stream_version INTEGER NULL,
  occurred_at    INTEGER NOT NULL,
  actor_id       INTEGER NOT NULL,
  actor_kind     TEXT NOT NULL,
  kind           TEXT NOT NULL,
  source         TEXT NOT NULL,
  correlation_id TEXT NULL,
  changes        TEXT NOT NULL,
  schema_version INTEGER NOT NULL DEFAULT 1,
  external_ref   TEXT NULL
);

CREATE UNIQUE INDEX events_stream_version
  ON events (stream_kind, stream_id, stream_version)
  WHERE stream_version IS NOT NULL;

CREATE UNIQUE INDEX events_external_ref
  ON events (external_ref)
  WHERE external_ref IS NOT NULL;

CREATE INDEX events_stream ON events (stream_kind, stream_id, id);
CREATE INDEX events_feed ON events (occurred_at, id);
CREATE INDEX events_actor ON events (actor_id, occurred_at);

CREATE TRIGGER events_no_update BEFORE UPDATE ON events
BEGIN
  SELECT RAISE(ABORT, 'events is append-only');
END;

CREATE TRIGGER events_no_delete BEFORE DELETE ON events
BEGIN
  SELECT RAISE(ABORT, 'events is append-only');
END;
```
