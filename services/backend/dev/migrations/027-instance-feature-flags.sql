BEGIN;

ALTER TABLE instances ADD COLUMN enable_osm_pull INTEGER NOT NULL DEFAULT 0;
ALTER TABLE instances ADD COLUMN enable_osm_push INTEGER NOT NULL DEFAULT 0;

UPDATE instances SET enable_osm_pull = COALESCE(osm_pull, 0);

ALTER TABLE instances DROP COLUMN osm_pull;

COMMIT;
