CREATE TABLE IF NOT EXISTS `chatbot_reports` (
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

CREATE INDEX IF NOT EXISTS `idx_chatbot_reports_created_at` ON `chatbot_reports` (`created_at`);
CREATE INDEX IF NOT EXISTS `idx_chatbot_reports_created_by` ON `chatbot_reports` (`created_by`);

CREATE TABLE IF NOT EXISTS `chatbot_report_photos` (
    `id` INTEGER PRIMARY KEY AUTOINCREMENT,
    `report_id` INTEGER NOT NULL,
    `photo_id` TEXT NOT NULL,
    FOREIGN KEY (`report_id`) REFERENCES `chatbot_reports` (`id`) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS `idx_chatbot_report_photos_report_id` ON `chatbot_report_photos` (`report_id`);
