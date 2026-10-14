BEGIN;

-- Add the new species management permission and grant it to editors.
-- Admins already receive every permission through the seed's
-- `SELECT 'admin', id FROM permissions` statement.
INSERT OR IGNORE INTO permissions (id) VALUES ('species:manage');
INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES ('editor', 'species:manage');

COMMIT;
