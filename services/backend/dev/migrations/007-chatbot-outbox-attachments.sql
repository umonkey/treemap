BEGIN;
ALTER TABLE `chatbot_outbox` ADD COLUMN `attachments` TEXT;
COMMIT;
