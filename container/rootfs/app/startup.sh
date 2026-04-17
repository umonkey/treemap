#!/bin/sh
set -e

SCHEMA="/app/schema-sqlite.sql"
DATABASE="/app/var/database.sqlite"

cd /app
mkdir -p /app/var/logs

# This is only needed for the dev server.
mkdir -p /app/static/_app/immutable

if [ ! -f $DATABASE ]; then
    echo "Initializing the database..."
    sqlite3 $DATABASE < $SCHEMA
fi

exec /usr/bin/supervisord -nc /etc/supervisord.conf
