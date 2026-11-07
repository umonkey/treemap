-- Universal file storage: files, variants, and link tables.
-- This migration is additive; legacy tables and columns stay in place.

CREATE TABLE IF NOT EXISTS files (
    id          INTEGER PRIMARY KEY,
    kind        TEXT NOT NULL,
    status      TEXT NOT NULL,
    is_visible  INTEGER NOT NULL DEFAULT 1,
    created_at  INTEGER NOT NULL,
    created_by  INTEGER NOT NULL,
    ready_at    INTEGER,
    deleted_at  INTEGER,
    deleted_by  INTEGER
);

CREATE TABLE IF NOT EXISTS files_variants (
    file_id     INTEGER NOT NULL,
    name        TEXT NOT NULL,
    key         TEXT NOT NULL,
    mime_type   TEXT NOT NULL,
    size_bytes  INTEGER NOT NULL,
    width       INTEGER,
    height      INTEGER,
    created_at  INTEGER NOT NULL,
    PRIMARY KEY (file_id, name)
);

CREATE TABLE IF NOT EXISTS trees_files (
    id          INTEGER PRIMARY KEY,
    tree_id     INTEGER NOT NULL,
    file_id     INTEGER NOT NULL,
    sort_order  INTEGER NOT NULL DEFAULT 0,
    added_at    INTEGER NOT NULL,
    added_by    INTEGER NOT NULL,
    deleted_at  INTEGER,
    deleted_by  INTEGER,
    UNIQUE (tree_id, file_id)
);

CREATE INDEX IF NOT EXISTS trees_files_tree_id ON trees_files (tree_id);

CREATE TABLE IF NOT EXISTS chatbot_alerts_files (
    id          INTEGER PRIMARY KEY,
    alert_id    INTEGER NOT NULL,
    file_id     INTEGER NOT NULL,
    added_at    INTEGER NOT NULL,
    added_by    INTEGER NOT NULL,
    UNIQUE (alert_id, file_id)
);

CREATE TABLE IF NOT EXISTS water_source_files (
    id              INTEGER PRIMARY KEY,
    water_source_id INTEGER NOT NULL,
    file_id         INTEGER NOT NULL,
    added_at        INTEGER NOT NULL,
    added_by        INTEGER NOT NULL,
    UNIQUE (water_source_id, file_id)
);

CREATE TABLE IF NOT EXISTS files_fallback (
    old_key  TEXT PRIMARY KEY,
    file_id  INTEGER NOT NULL,
    variant  TEXT NOT NULL
);

ALTER TABLE trees ADD COLUMN thumbnail_file_id INTEGER;
