-- SQLite database schema for the tree map.
-- Use `make sqlite-schema` to apply.
-- See this page for docs: https://github.com/umonkey/treemap/wiki/Database-structure

CREATE TABLE IF NOT EXISTS trees (
    `id` INT NOT NULL,
    `osm_id` INT NULL,
    `lat` REAL NOT NULL,
    `lon` REAL NOT NULL,
    `species` TEXT NOT NULL, -- latin name
    `notes` TEXT NULL,
    `height` REAL NULL,
    `circumference` REAL NULL,
    `diameter` REAL NULL,
    `state` TEXT NOT NULL DEFAULT "healthy",
    `added_at` INT NOT NULL,
    `updated_at` INT NOT NULL,
    `added_by` INT NOT NULL,
    `thumbnail_id` INT NULL,
    `year` INT NULL,
    `address` TEXT NULL,
    PRIMARY KEY(`id`)
);

CREATE INDEX IF NOT EXISTS trees_lat ON trees (lat);
CREATE INDEX IF NOT EXISTS trees_lon ON trees (lon);
CREATE UNIQUE INDEX IF NOT EXISTS trees_osm_id ON trees (osm_id);
CREATE INDEX IF NOT EXISTS trees_state ON trees (state);


-- Tree attributes.
-- This contains random properties, like year of planting, height, etc.
--
-- We keep all records ever added, we then aggregate them to use the latest
-- added value for each attribute to show on the tree page.
--
-- The code that reads or updates this table needs to be aware of the data
-- type, since the values are stored as strings.
CREATE TABLE IF NOT EXISTS trees_props (
    `id` INT NOT NULL,
    `tree_id` INT NOT NULL,
    `added_at` INT NOT NULL,
    `added_by` INT NOT NULL,
    `name` TEXT NOT NULL,
    `value` TEXT NOT NULL,
    PRIMARY KEY(`id`)
);

CREATE INDEX IF NOT EXISTS trees_props_tree_id ON trees_props (tree_id);


-- Tree images.
CREATE TABLE IF NOT EXISTS files (
    `id` INT NOT NULL,
    `tree_id` INT NOT NULL,
    `added_at` INT NOT NULL,
    `added_by` INT NOT NULL,
    `deleted_at` INT NULL,
    `deleted_by` INT NULL,
    `small_id` INT NOT NULL,
    `large_id` INT NOT NULL,
    PRIMARY KEY(`id`)
);

CREATE INDEX IF NOT EXISTS files_tree_id ON files (tree_id);


-- Upload tickets.
-- Ticket id is also the file name.
CREATE TABLE IF NOT EXISTS upload_tickets (
    `id` INT NOT NULL,
    `created_at` INT NOT NULL,
    `created_by` INT NOT NULL,
    `upload_url` TEXT NOT NULL,
    PRIMARY KEY(`id`)
);


-- User accounts.
CREATE TABLE IF NOT EXISTS users (
    `id` INT NOT NULL,
    `email` TEXT NOT NULL,
    `name` TEXT NOT NULL,
    `picture` TEXT NOT NULL,
    `trees_count` INT NOT NULL DEFAULT '0',
    `comments_count` INT NOT NULL DEFAULT '0',
    `updates_count` INT NOT NULL DEFAULT '0',
    `files_count` INT NOT NULL DEFAULT '0',
    PRIMARY KEY(`id`)
);
CREATE UNIQUE INDEX IF NOT EXISTS users_email ON users (email);


CREATE TABLE IF NOT EXISTS comments (
    `id` INT NOT NULL,
    `tree_id` INT NOT NULL,
    `added_at` INT NOT NULL,
    `added_by` INT NOT NULL,
    `message` TEXT NOT NULL,
    PRIMARY KEY(`id`)
);
CREATE INDEX IF NOT EXISTS comments_tree_id ON comments (tree_id);
CREATE INDEX IF NOT EXISTS comments_added_at ON comments (added_at);


CREATE TABLE IF NOT EXISTS queue_messages (
    `id` INT NOT NULL,
    `added_at` INT NOT NULL,
    `available_at` INT NOT NULL,
    `payload` TEXT NOT NULL,
    `attempts` INT NOT NULL DEFAULT '0',
    PRIMARY KEY(`id`)
);
CREATE INDEX IF NOT EXISTS queue_messages_added_at ON queue_messages (added_at);
CREATE INDEX IF NOT EXISTS queue_messages_available_at ON queue_messages (available_at);
CREATE INDEX IF NOT EXISTS queue_messages_attempts ON queue_messages (attempts);


CREATE TABLE IF NOT EXISTS species (
    `name` TEXT NOT NULL,
    `local` TEXT NOT NULL,
    `keywords` TEXT NULL,
    `wikidata_id` TEXT NOT NULL,
    PRIMARY KEY(`name`)
);


CREATE TABLE IF NOT EXISTS osm_trees (
    `id` INT UNSIGNED NOT NULL,
    `lat` REAL NOT NULL,
    `lon` REAL NOT NULL,
    `genus` TEXT NULL,
    `species` TEXT NULL,
    `species_wikidata` TEXT NULL,
    `height` REAL NULL,
    `circumference` REAL NULL,
    `diameter_crown` REAL NULL,
    PRIMARY KEY(`id`)
);

CREATE TABLE IF NOT EXISTS likes (
    `tree_id` INT NOT NULL,
    `user_id` INT NOT NULL,
    `state` INT NOT NULL DEFAULT '0',
    `updated_at` INT UNSIGNED NOT NULL,
    PRIMARY KEY(`tree_id`, `user_id`)
);
CREATE INDEX IF NOT EXISTS likes_state ON likes (state);

CREATE TABLE IF NOT EXISTS training (
    `id` INT NOT NULL,
    `user_id` INT NOT NULL,
    `added_at` INT NOT NULL,
    `result` REAL NOT NULL,
    PRIMARY KEY(`id`)
);
CREATE INDEX IF NOT EXISTS IDX_training_user_id ON training (user_id);
CREATE INDEX IF NOT EXISTS IDX_training_added_at ON training (added_at);
