-- Object storage inventory: one row per object seen in the configured buckets.
-- Rows for deleted objects are retained and marked with missing_since.

BEGIN;

CREATE TABLE IF NOT EXISTS `storage_objects` (
    `bucket`           TEXT NOT NULL,
    `key`              TEXT NOT NULL,
    `size_bytes`       INTEGER NOT NULL,
    `etag`             TEXT,
    `last_modified_at` INTEGER NOT NULL,
    `first_seen_at`    INTEGER NOT NULL,
    `last_seen_at`     INTEGER NOT NULL,
    `missing_since`    INTEGER,
    PRIMARY KEY (`bucket`, `key`)
);

CREATE INDEX IF NOT EXISTS `idx_storage_objects_missing_since`
    ON `storage_objects` (`missing_since`);

COMMIT;
