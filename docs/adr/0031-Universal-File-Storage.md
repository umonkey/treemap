# ADR 0031: Universal File Storage

- Date: 2026-11-06
- Status: accepted

## Context

Uploads grew feature by feature. Tree photos used `uploads` as a temporary ticket table and `trees_images` to hold three bare S3 keys (`source_id`, `small_id`, `large_id`). Telegram alert photos used `chatbot_alerts_photos` with a path. Water sources had no photos. Each path reimplemented key naming, ownership checks, and response shaping, and the API returned backend-proxied numeric ids rather than stable URLs.

The old pipeline also had correctness problems. The thumbnailer returned the original bytes when an image was already small, so EXIF metadata, including GPS coordinates, could be served under a `.jpg` key that was not actually JPEG. Several endpoints lacked ownership or completion checks.

We need one file domain that any feature can attach to, with explicit derived variants, stable public URLs, and a migration that does not require downtime.

## Decision

We introduce a universal file storage model with three parts: a logical file record, per-object variant records, and link tables that attach files to features.

Key characteristics:

- `files` holds identity and lifecycle only: `id`, `kind` (`photo`, `video`, `other`), `status` (`pending`, `processing`, `ready`, `failed`), `is_visible`, `created_at`, `created_by`, `ready_at`, `deleted_at`, `deleted_by`.
- `files_variants` holds one row per stored object: `file_id`, `name`, `key`, `mime_type`, `size_bytes`, `width`, `height`, `created_at`. The primary key is `(file_id, name)`.
- Link tables `trees_files`, `chatbot_alerts_files`, and `water_source_files` attach a file to a feature. `trees_files` carries `sort_order` and soft-delete columns; the other two are append-only.
- Objects live under `files/{id}/{name}.{ext}`. The original is uploaded to `files/{id}/original.{ext}`; derived images are written to `files/{id}/small.jpg` and `files/{id}/large.jpg`.
- `trees.thumbnail_file_id` references `files.id`. The legacy `trees.thumbnail_id` column is retained during the transition.

### The original is a variant

The original upload is recorded as a `files_variants` row named `original`, not as columns on `files`. This keeps all per-object metadata in one place, so mime type, byte size, and dimensions are read the same way for the original and for every derived image. Adding a new variant, such as a video poster, needs no change to `files`. It also means the original participates in the same URL construction, visibility filtering, and migration logic as the derived images.

`files` therefore stays a small identity and lifecycle record. It does not duplicate mime type, size, or dimensions.

### Upload and processing

`POST /v1/upload` accepts `{ size, content_type, kind? }`, creates a `pending` file, and returns a presigned PUT URL for `files/{id}/original.{ext}`. `POST /v1/upload/{id}/finish` requires the owner, moves the file to `processing`, and enqueues `ProcessFile`.

The worker reads the original, decodes it, applies EXIF orientation, strips metadata, and writes two JPEG variants: `small` with a longest side of up to 480 pixels and `large` with a longest side of up to 2,000 pixels. It records both variants, sets the original's dimensions, and marks the file `ready`. A failure marks the file `failed`. Both derived variants are always re-encoded, which fixes the metadata leak and guarantees that a `.jpg` key contains JPEG.

HEIC and HEIF are rejected at ticket time because the image decoder cannot read them.

### Stable CDN URLs

API responses carry public URLs built from the configured `files_base_url` and the variant key. `FileRead` exposes `urls: { original, large, small }`, `kind`, `width`, and `height`; `TreeRead` exposes `thumbnail_url`. The legacy `small_id`, `large_id`, and `source_id` fields remain populated for older clients.

The URLs are not presigned. Presigned URLs expire, defeat long-lived caching, and force every client to refresh them. The URLs are also not proxied through the API, which would spend backend bandwidth and latency on object delivery. File ids are unique and variant keys are never rewritten, so the objects are immutable and can be cached for a year.

### Delisting instead of hard removal

`files.is_visible` defaults to true. Setting it to false removes the file from API responses: a file is listable only when it is `ready`, visible, and not soft-deleted. The CDN URL may still resolve, because the object remains in a public bucket and may already be cached.

This is a deliberate v1 trade-off. A hard takedown would need a private bucket or prefix, signed CDN access, cache invalidation, and an object-deletion workflow. Those are operational and infrastructure changes that the current deployment does not have. Manual object removal remains an operational step when a file must disappear completely.

### Zero-downtime migration

Migration `028-file-storage.sql` is purely additive. It creates the new tables and adds `trees.thumbnail_file_id`; it does not drop or rewrite legacy tables or columns.

Apply `028-file-storage.sql` with the existing manual migration process. It is additive.

`bin/treemap migrate-files` backfills `trees_images` into the new tables. It reuses `source_id` as the file id when present, falling back to the row id. It inserts the file, the `original`, `small`, and `large` variants, the `trees_files` link, and `files_fallback` rows that map the old bare keys to the new file and variant. It copies objects server-side with `CopyObject`, skips missing or zero keys, and is idempotent and resumable in batches. It then sets `trees.thumbnail_file_id` by mapping the old `thumbnail_id` through the `small` fallback.

The legacy `GET /v1/files/{id}.jpg` route resolves `files_fallback` first, then the `small` variant of a file with that id, then the bare legacy key. Old clients keep working while the frontend moves to the URL fields. Legacy columns and objects stay in place until the cleanup issue ([#313](https://github.com/umonkey/treemap/issues/313)) is executed.

## Alternatives considered

- Keep per-feature tables and add columns: each feature would keep its own key naming, ownership checks, and response shape, and moderation would stay inconsistent.
- Store variant metadata as JSON on `files`: no per-variant queries or constraints, and every new variant would change the JSON shape.
- Presigned URLs in API responses: expiry and cache churn, plus refresh logic in every client.
- Private bucket with signed CDN URLs: a real takedown path, but it needs CDN signing, key management, and invalidation. Deferred until the deployment needs it.
- Hard delete on moderation: object cleanup, referential integrity, and audit questions that a visibility flag avoids.
- Big-bang migration: downtime and no rollback. The additive schema and compatibility route avoid both.

## Consequences

- One upload and processing pipeline serves trees, alerts, and water sources. A new feature attaches files with a link table and reuses the same URLs and visibility rules.
- Storage grows: each photo keeps the original plus two derived JPEGs. The original is retained for re-processing and for full-resolution display.
- `is_visible` is a delisting control, not a hard takedown. A reader with the CDN URL may still fetch a hidden object until it is removed manually.
- The legacy route, `trees_images`, `uploads`, `chatbot_alerts_photos`, and `files_fallback` remain until the cleanup issue ([#313](https://github.com/umonkey/treemap/issues/313)) is executed. `files_fallback` is migration-only and should not be used by new code.
- Video is schema-ready only. `kind` and the `poster` variant name exist, but there is no transcoding or poster generation in this change.
- There is no deduplication and no content hash. Uploading the same bytes twice creates two files.
- The file backfill is a separate CLI command, so operators can run it independently of the schema change.
- `trees.thumbnail_id` no longer means a `small_id`; new code reads `thumbnail_file_id`. The old column is only a migration input.
