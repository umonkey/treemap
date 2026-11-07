---
name: database-migrations
description: Checklist for changing the SQLite schema, adding migrations, permissions, or shared tables across the backend and chatbot. Use when editing services/backend/dev/migrations/**, services/backend/dev/schema-sqlite.sql, or a table the chatbot also reads.
---

# Database Migrations

## Contract

- Schema changes are raw, forward-only SQL files in `services/backend/dev/migrations/NNN-name.sql`, applied manually with the existing database tooling.
- Do not add a general migration runner or a `schema_migrations` table. Feature-specific backfills are separate, idempotent CLI commands such as `migrate-files`.
- The backend and chatbot share one SQLite database. If a table is read by both (`chatbot_alerts`, `chatbot_alerts_photos`, `trees`, `instances`), update both services' models and repositories in the same change.

## SQLite constraints

- SQLite cannot change a column's nullability in place. Rebuild the table in the `006`/`024` style: create the new table, copy the rows, drop the old table, rename the new one, and recreate every index.
- The application never enables `PRAGMA foreign_keys`, so `DROP TABLE` does not cascade to child tables. Do not enable it in a migration. `PRAGMA defer_foreign_keys` does not suppress `ON DELETE CASCADE`.
- Keep `services/backend/dev/schema-sqlite.sql` in sync with the migrations; it is the snapshot used by tests and local setup.

## Permissions

- Add the permission to `permissions`, grant it in `role_permissions`, and update `services/backend/dev/seed-sqlite.sql`.
- `admin` bypasses checks at runtime, but `/v1/me` only reports explicit grants. Persist the `admin` grant or the UI hides the feature from admins.

## Procedure

1. Write the migration and update the schema snapshot.
2. Update the backend domain model, repository, schemas, and service.
3. If the table is shared, update the chatbot model, repository, and any daemon that reads it.
4. Add or update the seed rows for permissions or fixtures.
5. Update the operator reference and add an ADR for a new pattern or irreversible trade-off.
6. Verify with `make -C services/backend check` and `make -C services/chatbot check`.
