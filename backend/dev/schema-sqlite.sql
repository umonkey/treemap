-- SQLite database schema for the tree map.
-- Use `make sqlite-schema` to apply.

CREATE TABLE IF NOT EXISTS trees (
    `id` INT NOT NULL,
    `lat` REAL NOT NULL,
    `lon` REAL NOT NULL,
    `name` TEXT NOT NULL,
    `height` REAL NULL,
    `circumference` REAL NULL,
    `diameter` REAL NULL,
    `state` TEXT NOT NULL DEFAULT "healthy",
    `added_at` INT NOT NULL,
    `updated_at` INT NOT NULL,
    `added_by` INT NOT NULL,
    PRIMARY KEY(`id`)
);

CREATE INDEX IF NOT EXISTS trees_lat ON trees (lat);
CREATE INDEX IF NOT EXISTS trees_lon ON trees (lon);
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
    PRIMARY KEY(`id`)
);

CREATE UNIQUE INDEX IF NOT EXISTS users_email ON users (email);


CREATE TABLE IF NOT EXISTS queue_messages (
    `id` INT NOT NULL,
    `added_at` INT NOT NULL,
    `available_at` INT NOT NULL,
    `payload` TEXT NOT NULL,
    PRIMARY KEY(`id`)
);
CREATE INDEX IF NOT EXISTS queue_messages_added_at ON queue_messages (added_at);
CREATE INDEX IF NOT EXISTS queue_messages_available_at ON queue_messages (available_at);
