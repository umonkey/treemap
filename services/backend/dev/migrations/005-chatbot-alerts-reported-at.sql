BEGIN;

ALTER TABLE `chatbot_alerts` ADD COLUMN `reported_at` INTEGER;

CREATE INDEX IF NOT EXISTS `idx_chatbot_alerts_reported_at` ON `chatbot_alerts` (`reported_at`);

CREATE TABLE IF NOT EXISTS `chatbot_outbox` (
    `id` INTEGER PRIMARY KEY AUTOINCREMENT,
    `alert_id` INTEGER NOT NULL,
    `chat_id` INTEGER NOT NULL,
    `text` TEXT NOT NULL,
    `status` TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'processing', 'sent', 'failed'
    `created_at` INTEGER NOT NULL,
    `sent_at` INTEGER,
    `next_retry_at` INTEGER NOT NULL,
    `attempts` INTEGER NOT NULL DEFAULT 0,
    `error_message` TEXT,
    FOREIGN KEY (`alert_id`) REFERENCES `chatbot_alerts` (`id`) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS `idx_chatbot_outbox_status_retry` ON `chatbot_outbox` (`status`, `next_retry_at`);

COMMIT;
