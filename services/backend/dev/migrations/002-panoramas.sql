BEGIN;

CREATE TABLE IF NOT EXISTS panoramas (
    `id` INT NOT NULL,
    `created_at` INT NOT NULL,
    `created_by` INT NOT NULL,
    `image_count` INT NOT NULL DEFAULT 0,
    `status` TEXT NOT NULL DEFAULT 'draft',
    `title` TEXT NOT NULL,
    `visible` INT NOT NULL DEFAULT 0,
    `has_video` INT NOT NULL DEFAULT 0,
    `has_track` INT NOT NULL DEFAULT 0,
    `has_web_video` INT NOT NULL DEFAULT 0,
    `video_timestamp` REAL NULL,
    PRIMARY KEY(`id`)
);

CREATE INDEX IF NOT EXISTS panoramas_created_at ON panoramas (created_at);

COMMIT;
