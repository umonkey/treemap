# Maintenance tasks

## Keeping file counts up to date

Users see their file counts on the profile page.
Those counts are updated as the files get uploaded or deleted.
However, if files are deleted from the database manually, the counter can go out of sync.
To fix it, use the following query:

```sql
UPDATE users SET files_count = (SELECT COUNT(1) FROM files WHERE files.added_by = users.id);
```

## Duplicate Resolution

The application can automatically identify and merge duplicate trees located within a meter of each other.

To run the merge process:

```bash
treemap merge-replaced-trees --confirm [limit]
```

Without `--confirm` the command only lists the pairs it would merge.

After merging, some local trees might be linked to OpenStreetMap nodes that were deleted on the OSM side. To fix these links, run:

```bash
treemap osm-remap-duplicates
```

See [Duplicates.md](./Duplicates.md) for more details.

## Anonymized data export

The application publishes an anonymized copy of the whole database for public research and analysis.

See [Database-export.md](./Database-export.md) for the output format, schedule, and verification steps.

## Storage inventory

The backend snapshots object storage into the `storage_objects` table so usage can be queried with SQL. Each run lists the files, backups, panoramas, and panorama source buckets, then records every object with its size, ETag, and modification time.

Run the sync manually with:

```bash
treemap storage-sync
```

It also runs nightly from `cron-daily`, after `backup-database` and `export-database`.

Column meanings:

- `first_seen_at`: Unix seconds when the object was first recorded; it does not change on later runs.
- `last_seen_at`: Unix seconds when the object was last present in a listing.
- `missing_since`: Unix seconds when the object was first absent from a listing, or NULL while it is present. It is cleared if the object reappears.

To report live usage per bucket:

```sql
SELECT bucket, COUNT(*) AS objects, SUM(size_bytes) AS bytes
FROM storage_objects
WHERE missing_since IS NULL
GROUP BY bucket;
```

Notes:

- `bucket` holds the physical bucket name from the configuration, not a logical role. The sync command logs each role with the bucket name it resolved.
- `last_modified_at` is the S3 `LastModified` value. A `CopyObject`, including a server-side copy used by a backfill, resets it to the time of the copy, so it is not the original upload time.
- `etag` is stored as returned. It is not an MD5 checksum for multipart uploads.
- Rows for objects deleted from a bucket are retained with `missing_since` set, so the table keeps the history of what disappeared. Filter on `missing_since IS NULL` to report only live objects. The table grows with the number of keys ever seen.
- The sync needs `s3:ListBucket` on the four buckets. It does not fetch objects.
