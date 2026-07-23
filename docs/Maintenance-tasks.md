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

The application can automatically identify and merge duplicate trees located at the same coordinates.

To run the merge process:

```bash
treemap merge-duplicates [limit]
```

After merging, some local trees might be linked to OpenStreetMap nodes that were deleted on the OSM side. To fix these links, run:

```bash
treemap osm-remap-duplicates
```

See [Duplicates.md](./Duplicates.md) for more details.
