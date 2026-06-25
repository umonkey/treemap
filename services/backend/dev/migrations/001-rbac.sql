BEGIN;

-- 1. Defined Roles
CREATE TABLE IF NOT EXISTS roles (
    `id` TEXT NOT NULL,
    PRIMARY KEY(`id`)
);

-- 2. Fine-grained Permissions
CREATE TABLE IF NOT EXISTS permissions (
    `id` TEXT NOT NULL,
    PRIMARY KEY(`id`)
);

-- 3. Role-Permission Mapping
CREATE TABLE IF NOT EXISTS role_permissions (
    `role_id` TEXT NOT NULL,
    `permission_id` TEXT NOT NULL,
    PRIMARY KEY(`role_id`, `permission_id`),
    FOREIGN KEY(`role_id`) REFERENCES roles(`id`) ON DELETE CASCADE,
    FOREIGN KEY(`permission_id`) REFERENCES permissions(`id`) ON DELETE CASCADE
);

-- 4. User-Role Mapping
CREATE TABLE IF NOT EXISTS user_roles (
    `user_id` INT NOT NULL,
    `role_id` TEXT NOT NULL,
    PRIMARY KEY(`user_id`, `role_id`),
    FOREIGN KEY(`user_id`) REFERENCES users(`id`) ON DELETE CASCADE,
    FOREIGN KEY(`role_id`) REFERENCES roles(`id`) ON DELETE CASCADE
);

COMMIT;
