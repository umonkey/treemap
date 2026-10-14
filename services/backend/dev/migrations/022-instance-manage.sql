BEGIN;

-- Add the instance management permission and grant it to admins only.
-- Admins already bypass permission checks, but /v1/me only reports
-- explicitly granted permissions, so the admin grant must be persisted for
-- the UI to surface the feature. Do NOT grant it to 'editor'.
INSERT OR IGNORE INTO permissions (id) VALUES ('instance:manage');
INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES ('admin', 'instance:manage');

COMMIT;
