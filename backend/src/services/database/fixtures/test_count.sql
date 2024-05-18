DELETE FROM trees;

INSERT
    INTO trees (id, lat, lon, species, added_at, updated_at, added_by, state)
    VALUES (1, 0.0, 0.0, 'Unknown', 0, 0, 0, 'healthy');

INSERT
    INTO trees (id, lat, lon, species, added_at, updated_at, added_by, state)
    VALUES (2, 0.0, 0.0, 'Unknown', 0, 0, 0, 'dead');

INSERT
    INTO trees (id, lat, lon, species, added_at, updated_at, added_by, state)
    VALUES (3, 0.0, 0.0, 'Unknown', 0, 0, 0, 'gone');
