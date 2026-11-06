BEGIN;

-- Allow administrators to edit any citizen feedback alert.
-- Admins already bypass permission checks, but /v1/me only reports
-- explicitly granted permissions, so the admin grant must be persisted.
INSERT OR IGNORE INTO permissions (id) VALUES ('alert:manage');
INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES ('admin', 'alert:manage');

COMMIT;
