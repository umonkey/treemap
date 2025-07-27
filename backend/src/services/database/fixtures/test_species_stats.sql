DELETE FROM trees;

INSERT INTO trees (id, lat, lon, species, state, added_at, added_by, updated_at) VALUES (1, 0, 0, 'Quercus robur', 'healthy', 0, 0, 0);

-- This has a space, which should be trimmed.
INSERT INTO trees (id, lat, lon, species, state, added_at, added_by, updated_at) VALUES (2, 0, 0, 'Quercus robur ', 'healthy', 0, 0, 0);

-- These should be gone, should be excluded.
INSERT INTO trees (id, lat, lon, species, state, added_at, added_by, updated_at) VALUES (3, 0, 0, 'Populus nigra', 'gone', 0, 0, 0);
INSERT INTO trees (id, lat, lon, species, state, added_at, added_by, updated_at) VALUES (4, 0, 0, 'Populus alba', 'stump', 0, 0, 0);
