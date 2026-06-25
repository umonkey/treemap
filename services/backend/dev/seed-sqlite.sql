-- SQLite database seed for the tree map.
-- Adds some sample data to test the API with.

INSERT INTO trees (id, lat, lon, species, notes, added_at, updated_at, updated_by, added_by) VALUES (1, 40.181389, 44.514444, 'Betula', 'An old birch', 1712853785, 1712853785, 1, 1);
INSERT INTO trees (id, lat, lon, species, notes, added_at, updated_at, updated_by, added_by) VALUES (2, 38.7749, -123.4194, 'Quercus', 'The Great Oak', 1712853785, 1712853785, 1, 1);
INSERT INTO trees (id, lat, lon, species, notes, added_at, updated_at, updated_by, added_by) VALUES (3, 39.7749, -124.4194, 'Pinus', 'Twin Pines', 1712853785, 1712853785, 1, 1);

-- Default RBAC data
INSERT OR IGNORE INTO roles (id) VALUES ('admin');
INSERT OR IGNORE INTO roles (id) VALUES ('editor');
INSERT OR IGNORE INTO roles (id) VALUES ('user');

INSERT OR IGNORE INTO permissions (id) VALUES ('comment:create');
INSERT OR IGNORE INTO permissions (id) VALUES ('comment:delete');
INSERT OR IGNORE INTO permissions (id) VALUES ('pano:edit');
INSERT OR IGNORE INTO permissions (id) VALUES ('tree:create');
INSERT OR IGNORE INTO permissions (id) VALUES ('tree:delete');
INSERT OR IGNORE INTO permissions (id) VALUES ('tree:edit');
INSERT OR IGNORE INTO permissions (id) VALUES ('user:manage');

INSERT OR IGNORE INTO role_permissions (role_id, permission_id) SELECT 'admin', id FROM permissions;
INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES ('editor', 'tree:create');
INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES ('editor', 'tree:edit');
INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES ('editor', 'comment:create');
INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES ('editor', 'pano:edit');
INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES ('user', 'tree:create');
INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES ('user', 'comment:create');
