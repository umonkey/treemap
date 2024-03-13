-- SQLite database schema for the tree map.
-- Use `make sqlite-schema` to apply.

CREATE TABLE IF NOT EXISTS trees (
    `id` INT NOT NULL,
    `lat` REAL NOT NULL,
    `lon` REAL NOT NULL,
    PRIMARY KEY(`id`)
);
