# ADR 0033: Comments on any entity

- Date: 2026-11-11
- Status: accepted

## Context

Comments were tied to a tree. `comments.tree_id` was `NOT NULL`, the repository read rows by tree, and `GET/POST /v1/trees/{id}/comments` were the only comment endpoints. The frontend kept the comment components under the tree route.

Water sources, panoramas, alerts, and future objects need the same discussion thread. A table per entity would duplicate the table, the repository, the API, and the UI, and every new entity would need a migration and a set of query changes. One table keyed by entity type and id keeps a single implementation and a single moderation surface.

## Decision

Store comments against a polymorphic `(entity_type, entity_id)` pair and expose them through one generic API.

Key characteristics:

- `comments` replaces `tree_id` with `entity_type TEXT NOT NULL` and `entity_id INT NOT NULL`. Migration `031-comments-entity.sql` rebuilds the table with the copy-and-drop pattern used by `006-chatbot-alerts-draft.sql` and `024-alert-web-submission.sql`, copies every existing row as `entity_type = 'tree'`, and creates `comments_entity` on `(entity_type, entity_id)` and `comments_added_at` on `(added_at)`.
- `entity_type` stays open `TEXT` with no `CHECK` constraint, so adding a type needs no schema migration. The backend models the known set as a closed `CommentEntity` enum that currently contains only `tree`. An unknown path value is rejected with `400 Bad Request`, so nothing can store an unknown type. An unknown stored value is a `500` database-structure error, because `031` rewrites every existing row to `tree` and the API is the only writer. Adding a type later is an enum variant plus loader support, not a migration.
- The repository reads and counts by both columns. The tree merger reassigns only rows with `entity_type = 'tree'`, so merging two trees cannot move a water source's comments.
- `GET/POST /v1/comments/{entity_type}/{id}` replace `GET/POST /v1/trees/{id}/comments`, which are removed. This is a breaking API change; the static frontend is the only in-repo client and is updated in the same change.
- Posting requires `comment:create`, the existing permission. Migration `031` inserts it and grants it to `admin`, `editor`, and `user`, because it previously existed only in the development seed.
- Reading stays public. `comment:read` is registered and granted to `admin`, `editor`, and `user`, but no handler enforces it. It exists so reads can be restricted later without renaming the permission.
- `trees.comment_count` remains tree-specific. A comment updates the denormalized count only when `entity_type = 'tree'`, and `recalculate_stats` filters on both columns.
- `comments.added_by` is nullable. A null author marks a system comment. The table rebuild in `031` drops the `NOT NULL` constraint, because SQLite cannot alter nullability in place. The public POST still requires `comment:create` and records the authenticated user; only an internal service path can omit the author.
- The frontend moves `Comment`, `CommentInput`, and `CommentForm` to `lib/components/comments/`. `CommentForm` takes an entity type and an id, performs its own request, and checks `comment:create` itself. A comment without an author renders a localized system label. No water, panorama, or alert surface renders them yet; this change provides the infrastructure.

Rejected alternatives:

- One comment table per entity: typed foreign keys and a natural `tree_id`, but it duplicates the table, repository, API, and UI for every entity, and a global recent-comments feed becomes a union of tables.
- `comment:write` as a new permission name: clearer semantics, but `comment:create` already exists, is already seeded, and means the same operation. A second name would leave two permissions for one action and require a migration to grant both.
- A database `CHECK` constraint for `entity_type`: enforces the known types in SQLite, but needs a migration and a table rebuild for every new entity, and SQLite cannot alter a check constraint without rebuilding the table. The backend enum gives the same rejection without touching the schema.
- Keeping the tree endpoints as aliases: avoids the break, but keeps two code paths and a tree-specific URL for a generic resource.

## Consequences

- One table, one repository, one API, and one set of components serve every entity, and adding an entity type needs an enum variant, loader support, and a call site, but no migration.
- The closed entity enum rejects unknown types on write: `POST /v1/comments/water/1` returns `400` instead of storing an orphan row. A comment whose stored `entity_type` is not a known variant makes the whole read fail with `500 DatabaseStructure`; only a manual edit can produce one.
- The database cannot enforce that `entity_id` points at a live row. Deleting an entity leaves its comments orphaned, as deleting a tree already did. A cleanup command can address them, and the missing foreign key is why no entity deletion changes.
- The two indexes cover the entity lookup and the recent-comments feed without a table scan.
- `comment:read` is unused until enforced. If reads become private later, both GET handlers and the frontend must add the gate.
- `trees.comment_count` is not derived from `comments`. A future entity that needs a count must add its own column or count on demand.
- A system comment has no author: it does not increment `users.comments_count` and leaves no actor in the audit trail, but it still counts toward `trees.comment_count` for a tree.
- Migration `031` must run before the new backend serves traffic. The old binary inserts `tree_id`, which the rebuilt table no longer has, and the new binary omits it.
- External clients of `GET/POST /v1/trees/{id}/comments` break with no compatibility alias.
