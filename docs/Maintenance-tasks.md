# Maintenance tasks

## Keeping file counts up to date

Users see their file counts on the profile page.
Those counts are updated as the files get uploaded or deleted.
However, if files are deleted from the database manually, the counter can go out of sync.
To fix it, use the following query:

``` sql
UPDATE users SET files_count = (SELECT COUNT(1) FROM files WHERE files.added_by = users.id);
```
