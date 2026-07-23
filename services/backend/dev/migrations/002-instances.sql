BEGIN;
CREATE TABLE instances (
    id INTEGER PRIMARY KEY,
    domain TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    email TEXT NOT NULL,
    enabled INTEGER NOT NULL DEFAULT 1
);

INSERT INTO instances VALUES (1, 'localhost', 'Trees of Yerevan [dev]', 'The local development version of the app.', 'vladimir@kanachyerevan.am', 1);
INSERT INTO instances VALUES (2, 'yerevan.treemaps.app', 'Trees of Yerevan', 'The public crowd-sourced map of all trees in Yerevan.', 'vladimir@kanachyerevan.am', 1);
COMMIT;
