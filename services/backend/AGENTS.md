# Trees of Yerevan: Backend

## Scope and Tech Stack

This directory contains the API, queue consumers and other backend code.

- The code is written in Rust 1.97.
- The web framework is Actix-Web.
- The database engine is SQLite.

## Architectural patterns

The API follows the Action-Domain-Responder model.

## Directory Structure

- src/actions: web actions. These extract input data from the request, pass it to the domain implementation, then convert the results back to the API response.
- src/cli: command line actions. These extract input data from the CLI arguments, pass it to the domain implementation, then present the results.
- src/domain: this is where the business logic lives, organized by domain.
- src/infra: infrastructure adapters. The code that talks to external services, like db, queue, file storage. Should not be interdependent, unless absolutely necessary.
- src/services: additional application logic which orchestrates domains. Does not have any domain specific knowledge.
- src/utils: common simple things with no dependencies.

## Coding Style

- Use the RequirePermission<> extractor to require certain permissions.
- Use macro-based routing (e.g., `#[get("/...")]`) for web actions.
- Action modules should define their router function (e.g., `pub fn tree_router(cfg: &mut ServiceConfig)`) in the `mod.rs` file, importing handlers from `actions.rs` via `pub use actions::*;`.

## Routing

Public API resources live under `/v1` (trees, alerts, files, upload, species, water, users, me, stats, streets, heatmap, comments, settings, duplicates, training). `/api` is reserved for operator and admin surfaces (panoramas, export, app settings, instance). Do not add a public resource under `/api`; join the existing `/v1` scope and its router.

## Permissions

- Declare a permission as a marker type implementing `Permission` in `src/services/app/require_permission.rs`, then gate the handler with `RequirePermission<P>`.
- A new permission needs a migration that inserts it into `permissions` and grants it in `role_permissions`, plus a matching `dev/seed-sqlite.sql` update.
- `admin` bypasses checks at runtime, but `/v1/me` only reports explicitly granted permissions. Persist the `admin` grant too, or the UI will hide the feature from admins.

## Queue Commands

A queued command exists in three hand-maintained places, and the compiler does not cross-check them:

1. The message struct, its `encode()`, and the `QueueCommand` variant in `src/services/queue_consumer/schemas.rs`.
2. The string decode arm in `src/services/queue_encoder/mod.rs`.
3. The processing arm in `src/services/queue_consumer/service.rs`.

A missing decoder arm compiles cleanly. `QueueCommand::decode` returns `Ok(None)` for an unknown command and `Err` for a malformed payload; `process_message` logs both and returns `Ok`, so the consumer deletes the message. The message is lost permanently. When adding or changing a command, update all three places and add a decode test. The all-commands decode test must cover every variant.

## File Storage

- New uploads write `files` and `files_variants`, then link through `trees_files`, `chatbot_alerts_files`, or `water_source_files`. The legacy cleanup ([#313](https://github.com/umonkey/treemap/issues/313)) is done: `trees_images`, `uploads`, and `files_fallback` are gone. `chatbot_alerts_photos` is the only remaining legacy table, kept read-only for old alert photos.
- A file is listable when `status = 'ready'`, `is_visible = 1`, and `deleted_at IS NULL`. Attaching accepts `pending` and `processing` files.
- Keys are `files/{id}/original.{ext}`, `files/{id}/small.jpg`, and `files/{id}/large.jpg`. S3-compatible storage does not infer `Content-Type`: every service-side write must set it, and `CopyObject` must use `MetadataDirective::Replace` when the source metadata is wrong.
- The `original` object's `Content-Type` comes from the client's PUT. The ticket `content_type` only selects the key extension and the `files_variants` row.
- Processing is idempotent; re-enqueueing a file is safe.
- Operator reference: `docs/features/files/README.md`.

## Database Migrations

Schema changes are raw forward-only SQL files in `dev/migrations/`, applied manually with the existing database tooling. Do not add a general migration runner or a `schema_migrations` workflow. Feature-specific backfills are separate, idempotent CLI commands such as `migrate-files`.

SQLite cannot change a column's nullability in place; rebuild the table in the `006`/`024` style (create new, copy, drop, rename, recreate indexes). The application never enables `PRAGMA foreign_keys`, so `DROP TABLE` does not cascade to child tables; do not enable it in a migration. `PRAGMA defer_foreign_keys` does not suppress `ON DELETE CASCADE`. Follow the `database-migrations` skill for the cross-service checklist.

## Useful commands

- `make build`: build the project (release mode).
- `make check`: run clippy with `-D warnings`, `cargo fmt --check`, `cargo check`, and the unit tests. This is the single verification gate; a separate `make test` is not required.
- `make format`: format the code.
- `make serve`: run the backend locally.
- `make test`: run unit tests.
- `make sqlite-seed`: install test data into a local SQLite database.

### CLI Commands

The binary also supports several CLI commands (run via `cargo run -- <command>`):

- `osm-pull`: get new trees from OpenStreetMap.
- `osm-push`: send new trees to OSM.
- `osm-push-changes`: send tree updates to OSM.
- `dispatch-queue`: run the queue consumer daemon.
- `scan-panoramas`: scan and calculate file sizes for panoramas.
- `serve`: run the web server.
- `storage-sync`: snapshot object storage inventory.
- `update-tree-address N`: update street address for a single tree.
- `update-tree-addresses`: update street address for all trees.
- `upload-files`: move local files to S3.
