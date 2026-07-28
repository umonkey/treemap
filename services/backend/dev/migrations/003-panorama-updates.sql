BEGIN;

ALTER TABLE panoramas ADD COLUMN failure_reason TEXT NULL;

UPDATE panoramas SET status = 'NEEDS_FILES' WHERE status = 'draft';
UPDATE panoramas SET status = 'SUCCESS' WHERE status = 'processed';

COMMIT;
