#!/bin/sh

# Local development startup wrapper.
#
# Initialize local application data, then delegate to the existing
# backend startup script without modifying its behavior.

set -eu

DATABASE="/app/var/database.sqlite"
SCHEMA="/app/schema-sqlite.sql"
BOOTSTRAP="/app/bootstrap-local.sql"

# Ensure the persistent database directory exists.
mkdir -p /app/var

# Create a new SQLite database only when it does not exist.
if [ ! -f "$DATABASE" ]; then
    echo "Initializing local SQLite database..."
    sqlite3 "$DATABASE" < "$SCHEMA"
fi

# Insert the required local instance and application permissions.
# The bootstrap SQL is idempotent and preserves existing records.
echo "Applying local database bootstrap..."
sqlite3 "$DATABASE" < "$BOOTSTRAP"

# Start the application using the existing backend startup script.
# That script launches Supervisor and all enabled processes.
exec /app/startup.sh