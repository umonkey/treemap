-- Initialize the minimum application data for local development.
--
-- This script does not insert any trees.
-- It is safe to run multiple times without duplicating records.

BEGIN;

-- Create a local instance with the default Yerevan map bounds.
-- Preserve existing instance settings on subsequent startups.
INSERT INTO instances (
    domain,
    name,
    description,
    email,
    enabled,
    bbox_w,
    bbox_s,
    bbox_e,
    bbox_n
)
VALUES (
    'localhost',
    'TreeMap Local',
    'Local development instance',
    'local@example.invalid',
    1,
    44.41675,
    40.11426,
    44.61444,
    40.24785
)
ON CONFLICT(domain) DO NOTHING;

-- Create the default application roles.
INSERT OR IGNORE INTO roles (id) VALUES
    ('admin'),
    ('editor'),
    ('user');

-- Create the default permissions.
INSERT OR IGNORE INTO permissions (id) VALUES
    ('comment:create'),
    ('comment:delete'),
    ('pano:edit'),
    ('tree:create'),
    ('tree:delete'),
    ('tree:edit'),
    ('user:manage'),
    ('water:manage'),
    ('species:manage'),
    ('instance:manage');

-- Grant every available permission to administrators.
INSERT OR IGNORE INTO role_permissions (
    role_id,
    permission_id
)
SELECT 'admin', id
FROM permissions;

-- Grant the standard permissions to editors and regular users.
INSERT OR IGNORE INTO role_permissions (
    role_id,
    permission_id
)
VALUES
    ('editor', 'tree:create'),
    ('editor', 'tree:edit'),
    ('editor', 'comment:create'),
    ('editor', 'pano:edit'),
    ('editor', 'water:manage'),
    ('editor', 'species:manage'),
    ('user', 'tree:create'),
    ('user', 'comment:create');

COMMIT;