BEGIN;

-- Replace the tree-only `tree_id` with a polymorphic (entity_type, entity_id)
-- pair so comments can belong to any object. Existing rows become tree comments.
-- `added_by` is nullable: a null author marks a comment written by the system.
CREATE TABLE `comments_new` (
    `id` INT NOT NULL,
    `entity_type` TEXT NOT NULL,
    `entity_id` INT NOT NULL,
    `added_at` INT NOT NULL,
    `added_by` INT NULL,
    `message` TEXT NOT NULL,
    PRIMARY KEY(`id`)
);

INSERT INTO `comments_new` (`id`, `entity_type`, `entity_id`, `added_at`, `added_by`, `message`)
SELECT `id`, 'tree', `tree_id`, `added_at`, `added_by`, `message`
FROM `comments`;

DROP TABLE `comments`;

ALTER TABLE `comments_new` RENAME TO `comments`;

CREATE INDEX IF NOT EXISTS `comments_entity` ON `comments` (`entity_type`, `entity_id`);
CREATE INDEX IF NOT EXISTS `comments_added_at` ON `comments` (`added_at`);

-- Posting a comment now requires `comment:create`. It existed only in the seed,
-- so insert it here as well; otherwise deployments built from migrations deny
-- everyone but admins.
-- `comment:read` is registered for future use; reads stay public.
-- Admins already bypass permission checks, but /v1/me only reports explicitly
-- granted permissions, so every admin grant must be persisted.
INSERT OR IGNORE INTO permissions (id) VALUES ('comment:create');
INSERT OR IGNORE INTO permissions (id) VALUES ('comment:read');
INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES ('admin', 'comment:create');
INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES ('editor', 'comment:create');
INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES ('user', 'comment:create');
INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES ('admin', 'comment:read');
INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES ('editor', 'comment:read');
INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES ('user', 'comment:read');

COMMIT;
