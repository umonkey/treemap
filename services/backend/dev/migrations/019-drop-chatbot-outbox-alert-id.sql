BEGIN;
ALTER TABLE `chatbot_outbox` DROP COLUMN `alert_id`;
COMMIT;