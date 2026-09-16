BEGIN;
ALTER TABLE `chatbot_outbox` ADD COLUMN `topic_id` INTEGER;
COMMIT;
