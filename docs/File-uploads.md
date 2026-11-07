# File Uploads

The platform stores every uploaded file once and attaches it to one or more features. This page is the operator reference for the storage layout, the upload and processing flow, the API shape, visibility, and the migration from the previous photo pipeline.

## Storage layout

Files live in a single S3-compatible bucket. Keys are derived from the file id and the variant name:

- `files/{id}/original.{ext}`: the uploaded bytes, unchanged.
- `files/{id}/small.jpg`: thumbnail, longest side up to 480 pixels.
- `files/{id}/large.jpg`: display image, longest side up to 2,000 pixels.
- `files/{id}/poster.jpg`: reserved for video posters; not written in this release.

The public base address is configured as `files_base_url` in `config.toml`. Production uses `https://treemap.fra1.cdn.digitaloceanspaces.com/`, so a variant key such as `files/123/small.jpg` becomes `https://treemap.fra1.cdn.digitaloceanspaces.com/files/123/small.jpg`. The base must not repeat the `files/` prefix. API responses join this base with the variant key. The URLs are stable and are not presigned.

Every object is written with an explicit `Content-Type` derived from its key extension (`image/jpeg` for the JPEG variants, `video/mp4` for MP4, and so on). S3-compatible storage does not infer it from the extension, and a server-side copy preserves the source metadata by default, so the backfill replaces it explicitly. A missing type makes the browser download the file instead of displaying it.

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

The migration is additive and runs while the service is live. It has two stages: create the new tables and backfill the existing tree images. The legacy `trees_images` rows, bare-key objects, and `uploads` table are not modified.

### Prerequisites

- A verified database backup.
- The `files_bucket`, `files_endpoint`, `files_region`, and `files_base_url` values in `config.toml`, plus the `FILES_KEY` and `FILES_SECRET` secrets.
- The release that contains `bin/treemap migrate-files`.
- A scratch database and a staging bucket for the rehearsal.

Run the command from the backend container or from any host that uses the same `config.toml` and secrets as the API. It initializes `AppState`, so it connects to the configured database and storage account.

### Rehearsal

1. Restore a production database dump into a scratch database and point a staging config at it.
2. Point `files_bucket` at a staging bucket that contains a representative sample of the production objects. The backfill reads the legacy keys and writes new keys; it does not delete anything.
3. Run the production steps below against the scratch database and the staging bucket.
4. Check the counts and sample the URLs. Resolve missing objects before touching production.

### Production

1. Back up the database and record the current object count in the files bucket.
2. Deploy the release. The API keeps the legacy response fields, so clients with cached bundles keep working until they reload.
3. Apply `028-file-storage.sql` with the usual database tooling. The migration is additive; it does not drop or rename anything. Apply it once. On a local SQLite database:

   ```bash
   sqlite3 var/database.sqlite < services/backend/dev/migrations/028-file-storage.sql
   ```

   The final `ALTER TABLE` statement fails if `trees.thumbnail_file_id` already exists. Skip that statement on a database built from the current `schema-sqlite.sql`.

4. Backfill the existing tree images:

   ```bash
   bin/treemap migrate-files
   ```

   The command reads `trees_images` in batches of 100 and creates the matching `files`, `files_variants`, `trees_files`, and `files_fallback` rows. It copies each legacy object to `files/{id}/{variant}.jpg` with a server-side `CopyObject`. Zero or missing legacy keys are skipped with a warning. When the loop finishes, it sets `trees.thumbnail_file_id` from the legacy `thumbnail_id` through the `small` fallback, and logs the number of migrated images. The command is idempotent: re-running it skips rows and objects that already exist, so it is safe to run again after an interruption. The backfill copies objects instead of moving them, so the bucket temporarily holds both layouts.

5. Run `bin/treemap migrate-files` once more to pick up any legacy rows written during the first pass.
6. Verify the result before freezing the legacy path.
7. Freeze writes to the legacy photo path. Do not delete the old objects or columns.

### Verification

Run these queries against the configured database:

```sql
-- One trees_files row per legacy tree image, unless duplicate legacy rows
-- were collapsed by the UNIQUE(tree_id, file_id) constraint.
SELECT (SELECT COUNT(*) FROM trees_images) AS legacy_images,
       (SELECT COUNT(*) FROM trees_files) AS migrated_links;

-- Every legacy key should have a fallback mapping.
SELECT COUNT(*) FROM files_fallback;

-- Backfilled variants carry placeholder metadata (zero size, null dimensions).
SELECT COUNT(*) FROM files_variants WHERE size_bytes = 0;

-- Trees that had a legacy thumbnail should have a file thumbnail.
SELECT COUNT(*) FROM trees
WHERE thumbnail_id IS NOT NULL AND thumbnail_file_id IS NULL;
```

Sample a few new keys and check the public URLs:

```sql
SELECT key FROM files_variants WHERE name = 'small' LIMIT 5;
```

```bash
curl -I "${FILES_BASE_URL}files/123/small.jpg"
```

Replace `FILES_BASE_URL` with the configured base address. Check that a legacy link still resolves through the fallback:

```bash
curl -I "https://api.treemaps.app/v1/files/163702859915988992.jpg"
```

### Rollback

Redeploy the previous release. The new tables, the copied objects, and the backfilled rows are inert to the old code. The legacy columns and objects are untouched, so the old route and the old tree responses keep working. Do not run the cleanup until the new release has been stable for the agreed grace period.

### After the grace period

The cleanup is tracked in issue [#313](https://github.com/umonkey/treemap/issues/313). It deletes the old bare-key objects, drops the legacy columns, drops `files_fallback`, and removes the compatibility route.

## Limitations

- Video is schema-ready only. There is no transcoding, poster generation, or playback pipeline.
- There is no deduplication and no content hash.
- HEIC and HEIF are rejected at ticket time.
- `is_visible` is a delisting control, not a hard takedown.
- The original keeps its metadata. Only the derived variants are stripped.
- The backfill writes every variant with a `.jpg` suffix and records `image/jpeg`, regardless of the legacy object's actual format. The copied bytes are unchanged.
- Backfilled variants carry placeholder metadata (zero size, null dimensions). The storage inventory job is tracked separately in issue [#312](https://github.com/umonkey/treemap/issues/312).
