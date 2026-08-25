BEGIN;

PRAGMA defer_foreign_keys = ON;

CREATE TABLE `chatbot_alerts_new` (
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
    `status` TEXT NOT NULL DEFAULT 'draft',
    `response_text` TEXT,
    `responded_at` INTEGER,
    `reported_at` INTEGER
);

INSERT INTO `chatbot_alerts_new` (id, created_at, created_by, chat_id, message_id, username, language_code, lat, lon, description, status, response_text, responded_at, reported_at)
SELECT id, created_at, created_by, chat_id, message_id, username, language_code, lat, lon, description, COALESCE(status, 'draft'), response_text, responded_at, reported_at
FROM `chatbot_alerts`;

DROP TABLE `chatbot_alerts`;

ALTER TABLE `chatbot_alerts_new` RENAME TO `chatbot_alerts`;

CREATE INDEX IF NOT EXISTS `idx_chatbot_alerts_created_at` ON `chatbot_alerts` (`created_at`);
CREATE INDEX IF NOT EXISTS `idx_chatbot_alerts_created_by` ON `chatbot_alerts` (`created_by`);
CREATE INDEX IF NOT EXISTS `idx_chatbot_alerts_reported_at` ON `chatbot_alerts` (`reported_at`);
CREATE INDEX IF NOT EXISTS `idx_chatbot_alerts_status_reported` ON `chatbot_alerts` (`status`, `reported_at`);

COMMIT;
