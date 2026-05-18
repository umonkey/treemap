CREATE TABLE IF NOT EXISTS `chatbot_alerts` (
    `id` INTEGER PRIMARY KEY AUTOINCREMENT,
    `created_at` INTEGER NOT NULL,
    `created_by` INTEGER NOT NULL,
    `chat_id` INTEGER NOT NULL,
    `message_id` INTEGER,
    `username` TEXT,
    `language_code` TEXT,
    `lat` REAL,
    `lon` REAL,
    `description` TEXT,
    `status` TEXT NOT NULL DEFAULT 'new',
    `response_text` TEXT,
    `responded_at` INTEGER
);

CREATE INDEX IF NOT EXISTS `idx_chatbot_alerts_created_at` ON `chatbot_alerts` (`created_at`);
CREATE INDEX IF NOT EXISTS `idx_chatbot_alerts_created_by` ON `chatbot_alerts` (`created_by`);

CREATE TABLE IF NOT EXISTS `chatbot_alerts_photos` (
    `id` INTEGER PRIMARY KEY AUTOINCREMENT,
    `alert_id` INTEGER NOT NULL,
    `photo_path` TEXT NOT NULL,
    FOREIGN KEY (`alert_id`) REFERENCES `chatbot_alerts` (`id`) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS `idx_chatbot_alerts_photos_alert_id` ON `chatbot_alerts_photos` (`alert_id`);
