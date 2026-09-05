BEGIN;

ALTER TABLE panoramas DROP COLUMN transcode_arn;
ALTER TABLE panoramas DROP COLUMN transcode_status;

COMMIT;
