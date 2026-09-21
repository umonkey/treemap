BEGIN;

CREATE TABLE `chatbot_outbox_new` (
    `id` INTEGER PRIMARY KEY AUTOINCREMENT,
    `chat_id` INTEGER NOT NULL,
    `topic_id` INTEGER,
    `text` TEXT NOT NULL,
    `attachments` TEXT,
    `status` TEXT NOT NULL DEFAULT 'pending',
    `created_at` INTEGER NOT NULL,
    `sent_at` INTEGER,
    `next_retry_at` INTEGER NOT NULL,
    `attempts` INTEGER NOT NULL DEFAULT 0,
    `error_message` TEXT
);

INSERT INTO `chatbot_outbox_new`
    (`id`, `chat_id`, `topic_id`, `text`, `attachments`, `status`,
     `created_at`, `sent_at`, `next_retry_at`, `attempts`, `error_message`)
SELECT
    `id`, `chat_id`, `topic_id`, `text`, `attachments`, `status`,
    `created_at`, `sent_at`, `next_retry_at`, `attempts`, `error_message`
FROM `chatbot_outbox`;

DROP TABLE `chatbot_outbox`;

ALTER TABLE `chatbot_outbox_new` RENAME TO `chatbot_outbox`;

CREATE INDEX IF NOT EXISTS `idx_chatbot_outbox_status_retry`
    ON `chatbot_outbox` (`status`, `next_retry_at`);

COMMIT;