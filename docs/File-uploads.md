# File Uploads

The platform stores every uploaded file once and attaches it to one or more features. This page is the operator reference for the storage layout, the upload and processing flow, the API shape, visibility, and the migration from the previous photo pipeline.

## Storage layout

Files live in a single S3-compatible bucket. Keys are derived from the file id and the variant name:

- `files/{id}/original.{ext}`: the uploaded bytes, unchanged.
- `files/{id}/small.jpg`: thumbnail, longest side up to 480 pixels.
- `files/{id}/large.jpg`: display image, longest side up to 2,000 pixels.
- `files/{id}/poster.jpg`: reserved for video posters; not written in this release.

The public base address is configured as `files_base_url` in `config.toml`. The default is `https://treemap.am/files/`. API responses join this base with the variant key. The URLs are stable and are not presigned.

## Data model

- `files`: one row per logical file. Holds `id`, `kind` (`photo`, `video`, `other`), `status` (`pending`, `processing`, `ready`, `failed`), `is_visible`, `created_at`, `created_by`, `ready_at`, `deleted_at`, `deleted_by`.
- `files_variants`: one row per stored object, keyed by `(file_id, name)`. Holds `key`, `mime_type`, `size_bytes`, `width`, `height`, `created_at`. The original is a variant named `original`.
- `trees_files`: attaches files to trees. Holds `sort_order` and soft-delete columns.
- `chatbot_alerts_files`: attaches files to citizen feedback alerts.
- `water_source_files`: attaches files to water sources.
- `files_fallback`: migration-only map from a legacy bare S3 key to `(file_id, variant)`. New code must not write to it.
- `trees.thumbnail_file_id`: references `files.id`. The legacy `trees.thumbnail_id` is retained as a migration input.

A file is listable when `status = 'ready'`, `is_visible = 1`, and `deleted_at IS NULL`.

## Upload workflow

1. The client calls `POST /v1/upload` with `{ "size": 12345, "content_type": "image/jpeg", "kind": "photo" }`. `kind` is optional and defaults to `photo`. The response is `{ "id": "<file id>", "url": "<presigned PUT URL>" }`.
2. The server inserts a `files` row with `status = 'pending'` and a `files_variants` row named `original` pointing at `files/{id}/original.{ext}`. HEIC and HEIF content types are rejected with a 400.
3. The client uploads the bytes to the presigned URL with a PUT.
4. The client calls `POST /v1/upload/{id}/finish`. The caller must own the file. The server sets `status = 'processing'` and enqueues `ProcessFile`. Finishing an already processing or ready file is a no-op.
5. The queue consumer reads the original, decodes it, applies EXIF orientation, strips metadata, and writes `small` and `large` as JPEG. It records both variants with their dimensions, updates the original's dimensions, and sets `status = 'ready'` and `ready_at`. On failure it sets `status = 'failed'`.
6. The client polls `GET /v1/files/{id}/status`, which returns `{ "ready": true }` once processing completes.

The local storage driver does not implement presigned uploads. Local development must use S3-compatible storage or a local S3 emulator.

## Attaching files

- Trees: `POST /v1/trees/{id}/photos` with `{ "files": ["<id>", ...] }`. The caller must own each file. The first attached file becomes the tree thumbnail when the tree has none. `PUT /v1/trees/{id}/thumbnail` with `{ "file": "<id>" }` sets it explicitly.
- Alerts: `POST /v1/alerts` accepts `files` alongside the description and coordinates. At least one and at most ten photos are required. `GET /v1/alerts/{id}/photos` returns the public URLs of the visible attached files.
- Water sources: `POST /v1/water` and `PATCH /v1/water/{id}` accept `files`. The caller must own each file.

Attachment validates that the file exists, is not deleted, and belongs to the caller. It does not require the file to be `ready`, so a client can attach while processing is still running.

## API responses

`FileRead` contains:

- `id`, `small_id`, `large_id`, `source_id`: compatibility fields. For new files, all three id fields carry the file id; legacy tree images keep their old ids during the transition.
- `added_at`, `added_by`.
- `urls`: `{ original, large, small }`, each a stable public URL or null when the variant is missing.
- `kind`, `width`, `height`.

`TreeRead` contains `thumbnail_url`, built from `thumbnail_file_id` when present and falling back to the legacy `thumbnail_id` during the transition.

## Visibility and deletion

`is_visible = 0` delists a file. It is omitted from tree, alert, and water source responses, and the legacy route still resolves it. The object remains in the bucket and its CDN URL may still resolve, including from caches. Removing the object is a manual operational step.

`DELETE /v1/files/{id}` soft-deletes a file owned by the caller by setting `deleted_at` and `deleted_by`. It does not delete objects. A soft-deleted file is not listable and cannot be attached.

## Legacy compatibility

`GET /v1/files/{id}.jpg` and `GET /v1/files/{id}` resolve in this order:

1. `files_fallback` for the bare key, then the mapped variant.
2. The `small` variant of a file with this id.
3. The bare legacy key in the bucket.

The legacy tables `uploads`, `trees_images`, and `chatbot_alerts_photos` remain in place. They are read-only compatibility paths and are removed by the cleanup issue ([#313](https://github.com/umonkey/treemap/issues/313)).

## Migration

The migration is additive and can run while the service is live.

1. Back up the database and record the current object count.
2. Deploy the release. Run `bin/treemap migrate --baseline` once to record migrations `001` to `027` as applied without executing them.
3. Run `bin/treemap migrate` to apply `028-file-storage.sql`.
4. Run `bin/treemap migrate-files` against a copy of the database and a staging bucket. Compare the `trees_images` count with the `trees_files` count and spot-check `files_fallback` lookups and sample URLs.
5. Run `bin/treemap migrate-files` in production. It is idempotent, batched, and resumable. It copies objects server-side with `CopyObject`, skips zero or missing keys, and sets `trees.thumbnail_file_id` from the old `thumbnail_id` through the `small` fallback.
6. Deploy the client that reads `urls` and `thumbnail_url`.
7. Verify that `trees_files` matches `trees_images`, that sample legacy links resolve, and that sample CDN URLs return 200.
8. Freeze writes to the legacy photo path. Do not delete old objects or columns.

Rollback is a redeploy of the previous release. The legacy tables, columns, and objects are untouched until the cleanup issue ([#313](https://github.com/umonkey/treemap/issues/313)) is executed.

## Limitations

- Video is schema-ready only. There is no transcoding, poster generation, or playback pipeline.
- There is no deduplication and no content hash.
- HEIC and HEIF are rejected at ticket time.
- `is_visible` is a delisting control, not a hard takedown.
- The original keeps its metadata. Only the derived variants are stripped.
