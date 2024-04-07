-- SQLite database schema for the tree map.
-- Use `make sqlite-schema` to apply.

CREATE TABLE IF NOT EXISTS trees (
    `id` INT NOT NULL,
    `lat` REAL NOT NULL,
    `lon` REAL NOT NULL,
    `name` TEXT NOT NULL,
    PRIMARY KEY(`id`)
);


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
CREATE TABLE IF NOT EXISTS trees_files (
    `id` INT NOT NULL,
    `tree_id` INT NOT NULL,
    `added_at` INT NOT NULL,
    `small_url` TEXT NOT NULL,
    `large_url` TEXT NOT NULL,
    PRIMARY KEY(`id`)
);

CREATE INDEX IF NOT EXISTS trees_files_tree_id ON trees_files (tree_id);


-- User accounts.

CREATE TABLE IF NOT EXISTS users (
    `id` INT NOT NULL,
    `email` TEXT NOT NULL,
    `name` TEXT NOT NULL,
    `picture` TEXT NOT NULL,
    PRIMARY KEY(`id`)
);

CREATE UNIQUE INDEX IF NOT EXISTS users_email ON users (email);
