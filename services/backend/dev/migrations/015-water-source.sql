BEGIN;

CREATE TABLE IF NOT EXISTS `water_source` (
    `id` INT NOT NULL PRIMARY KEY,
    `created_at` INT NOT NULL,
    `created_by` INT NOT NULL,
    `updated_at` INT NOT NULL,
    `lat` REAL NOT NULL,
    `lon` REAL NOT NULL,
    `status` TEXT NOT NULL DEFAULT 'operational'
);

CREATE INDEX IF NOT EXISTS `idx_water_source_lat` ON `water_source` (`lat`);
CREATE INDEX IF NOT EXISTS `idx_water_source_lon` ON `water_source` (`lon`);
CREATE INDEX IF NOT EXISTS `idx_water_source_status` ON `water_source` (`status`);

COMMIT;