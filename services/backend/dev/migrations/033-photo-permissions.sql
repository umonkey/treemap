BEGIN;

-- Dedicated tree photo permissions replace the tree-wide `tree:edit` gate on
-- photo surfaces, so the standard contributor role can upload photos without
-- receiving edit rights over the whole tree.
--
-- `photo:read` gates the frontend Photos tab; tree reads are public and return
-- photos already, so no backend handler uses it. `photo:create` gates attaching
-- photos to a tree. `photo:manage` gates setting a thumbnail and deleting a file.
--
-- Admins bypass permission checks at runtime, but /v1/me only reports explicitly
-- granted permissions, so every admin grant must be persisted. The `viewer` role
-- is read-only and is not assigned to a user row; it is the anonymous fallback.
INSERT OR IGNORE INTO permissions (id) VALUES ('photo:read');
INSERT OR IGNORE INTO permissions (id) VALUES ('photo:create');
INSERT OR IGNORE INTO permissions (id) VALUES ('photo:manage');

INSERT OR IGNORE INTO roles (id) VALUES ('viewer');

INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES ('admin', 'photo:read');
INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES ('admin', 'photo:create');
INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES ('admin', 'photo:manage');
INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES ('editor', 'photo:read');
INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES ('editor', 'photo:create');
INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES ('editor', 'photo:manage');
INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES ('user', 'photo:read');
INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES ('user', 'photo:create');
INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES ('viewer', 'photo:read');
INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES ('viewer', 'comment:read');

COMMIT;
