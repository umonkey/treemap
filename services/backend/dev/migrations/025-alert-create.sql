BEGIN;

-- Allow signed-in users to submit alerts from the web UI.
-- Admins already bypass permission checks, but /v1/me only reports
-- explicitly granted permissions, so the admin grant must be persisted.
INSERT OR IGNORE INTO permissions (id) VALUES ('alert:create');
INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES ('admin', 'alert:create');
INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES ('editor', 'alert:create');
INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES ('user', 'alert:create');

COMMIT;
