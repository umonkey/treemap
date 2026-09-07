BEGIN;
ALTER TABLE panoramas ADD COLUMN processing_time INT NULL;
COMMIT;
