BEGIN;

CREATE TABLE IF NOT EXISTS panoramas (
    `id` INT NOT NULL,
    `storage_key` TEXT NULL,
    `created_at` INT NOT NULL,
    `created_by` INT NOT NULL,
    `image_count` INT NOT NULL DEFAULT 0,
    `status` TEXT NOT NULL DEFAULT 'NEEDS_FILES',
    `title` TEXT NOT NULL,
    `visible` INT NOT NULL DEFAULT 0,
    `source_video_path` TEXT NULL,
    `gpx_path` TEXT NULL,
    `web_video_path` TEXT NULL,
    `transcode_arn` TEXT NULL,
    `transcode_status` TEXT NULL,
    `video_timestamp` REAL NULL,
    `gpx_offset` REAL NULL,
    `lat_offset` REAL NOT NULL DEFAULT 0.0,
    `lon_offset` REAL NOT NULL DEFAULT 0.0,
    `processing_arn` TEXT NULL,
    `processing_status` TEXT NULL,
    `failure_reason` TEXT NULL,
    `min_lat` REAL NULL,
    `max_lat` REAL NULL,
    `min_lon` REAL NULL,
    `max_lon` REAL NULL,
    `points_json` TEXT NULL,
    PRIMARY KEY(`id`)
);

CREATE INDEX IF NOT EXISTS panoramas_created_at ON panoramas (created_at);


CREATE TABLE IF NOT EXISTS panoramas_images (
    id INT NOT NULL,
    panorama_id INT NOT NULL,
    filename TEXT NOT NULL,
    lat REAL NOT NULL,
    lng REAL NOT NULL,
    heading REAL NOT NULL,
    pitch REAL NOT NULL,
    roll REAL NOT NULL,
    hidden INT NOT NULL DEFAULT 0,
    PRIMARY KEY(id)
);

CREATE INDEX IF NOT EXISTS panoramas_images_panorama_id ON panoramas_images (panorama_id);

CREATE TABLE IF NOT EXISTS panoramas_hints (
    `image_id` INT NOT NULL,
    `angle` REAL NOT NULL,
    `user_id` INT NOT NULL
);
CREATE INDEX IF NOT EXISTS panoramas_hints_image_id ON panoramas_hints (image_id);

DROP TABLE IF EXISTS mapillary_images;
DROP TABLE IF EXISTS mapillary_sequences;
DROP TABLE IF EXISTS mapillary_trees;

COMMIT;
