# Database export

## Overview

The application publishes an anonymized copy of the tree database for public research and analysis. The export is a gzipped SQLite file that contains no personally identifiable information.

The output is written to `export/YYYY-MM-DD.sqlite.gz` in the backup bucket.

## Running manually

Run the export command on the server host:

```bash
treemap export-database
```

The command requires the same configuration and secrets as the server, including access to the backup bucket.

## Default schedule

The export runs daily at 04:00 as part of the `cron-daily` script, immediately after `backup-database`.

- Script: `services/backend/docker/rootfs/app/bin/cron-daily`.
- Crontab: `services/backend/docker/rootfs/etc/crontabs/root`.

## Exported tables

The export contains four tables.

- `trees`: id, osm_id, added_at, updated_at, lat, lon, species, state, height, diameter, circumference.
- `trees_images`: id, tree_id, added_at, url.
- `observations`: id, tree_id, created_at, and the boolean condition flags.
- `water_source`: id, created_at, updated_at, status, lat, lon.

## Anonymization

The export removes all fields that can identify users or expose free-form text.

- Removed: user ids such as `added_by` and `updated_by`, tree `notes`, tree `address`, and all change history props.
- Retained: coordinates, OSM ids, species, measurements, and timestamps.

## Downloading and verifying

Download the latest export and inspect it locally:

```bash
aws s3 cp s3://<backup-bucket>/export/YYYY-MM-DD.sqlite.gz .
gunzip -c YYYY-MM-DD.sqlite.gz > export.sqlite
sqlite3 export.sqlite '.tables'
```

## Implementation notes

The export is produced by `ExportClient`, which writes into a temporary SQLite file using a transaction and `journal_mode = MEMORY` to avoid leaving a `-wal` sidecar that would result in an incomplete archive.

- Source: `services/backend/src/infra/export/client.rs`.
