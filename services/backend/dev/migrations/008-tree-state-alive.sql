BEGIN;
UPDATE `trees` SET `state` = 'alive' WHERE `state` IN ('sick', 'deformed', 'healthy');
UPDATE `trees_props` SET `value` = 'alive' WHERE `name` = 'state' AND `value` IN ('sick', 'deformed', 'healthy');
COMMIT;
