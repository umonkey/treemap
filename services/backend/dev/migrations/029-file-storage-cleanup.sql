-- Remove the legacy file storage compatibility layer (#313).
-- Run after the legacy S3 objects have been deleted; see docs/features/files/README.md.

BEGIN;

DROP INDEX IF EXISTS trees_images_tree_id;
DROP TABLE IF EXISTS trees_images;
DROP TABLE IF EXISTS uploads;
DROP TABLE IF EXISTS files_fallback;

ALTER TABLE trees DROP COLUMN thumbnail_id;

COMMIT;
