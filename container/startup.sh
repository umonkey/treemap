#!/bin/sh
set -e

SCHEMA="/app/schema-sqlite.sql"
DATABASE="/app/var/database.sqlite"
BINARY="/app/bin/treemap"

cd /app
mkdir -p /app/var

if [ ! -f $DATABASE ]; then
    echo "Initializing the database..."
    sqlite3 $DATABASE < $SCHEMA
fi

$BINARY --queue-consumer &
$BINARY &

wait -n
exit $?
