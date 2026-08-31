BEGIN;

ALTER TABLE `chatbot_alerts` ADD COLUMN `ping_at` INTEGER;

CREATE INDEX IF NOT EXISTS `idx_chatbot_alerts_status_ping` ON `chatbot_alerts` (`status`, `ping_at`);

COMMIT;
