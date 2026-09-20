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
treemap merge-duplicates --confirm [limit]
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
