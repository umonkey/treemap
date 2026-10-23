# Developement hints

## Running the application

### Existing development environment

The standard development environment uses `compose.yaml` and the backend configuration in `services/backend/config.toml.dev`.

Start it with:

```bash
docker compose up
```

This environment retains its existing cloud integrations and scheduled tasks.

### Isolated local development

The isolated local environment is intended for contributors who want to run TreeMap without configuring AWS SQS or S3-compatible storage.

Start the application:

```bash
make start-local
```

This command initializes local secrets and starts the backend and frontend using both `compose.yaml` and `compose.local.yaml`.

Without Make:

```bash
sh tools/init-local.sh
docker compose -f compose.yaml -f compose.local.yaml up --build
```

The frontend is available at http://localhost:5173 and the backend API at http://localhost:8000.

### Local configuration

The local setup uses `services/backend/config.toml.local`, selected through the `TREEMAP_CONFIG` environment variable.

The local Compose file overrides only the settings required for an isolated environment. The original development and production configurations are unchanged.

The local environment uses a separate Compose project named `trees_local`.

### Local database

The database is stored at `/app/var/database.sqlite` inside the backend container, using a persistent Docker volume.

On the first startup, the local initialization script creates the database schema and inserts the required instance and permission records.

No trees are inserted automatically.

Open the SQLite console:

```bash
make sql-local
```

Check the number of trees:

```sql
SELECT COUNT(*) FROM trees;
```

The expected result for a fresh local database is `0`.

### Scheduled tasks

Scheduled tasks are disabled in the isolated local environment through a Supervisor configuration override.

This prevents automatic synchronization with OpenStreetMap.

The standard development environment retains its existing scheduled tasks.

The SQLite queue dispatcher and other Supervisor-managed processes remain enabled.

### Local secrets

Local credentials are stored under `.local-secrets/`.

The directory is created automatically by:

```bash
make local-init
```

Existing secret files are never overwritten.

These files contain development placeholders and must not be used for production deployments.

The local backend still requires placeholder AWS credentials to initialize its AWS Batch client, even though AWS Batch is not used for normal local development.

### Frontend dependencies

The local Compose configuration mounts a dedicated Docker volume at `/app/node_modules`.

Dependencies are initially provided by the frontend Docker image.

If dependencies become outdated after changing `package-lock.json`, reinstall them with:

```bash
docker compose -f compose.yaml -f compose.local.yaml run --rm --no-deps frontend npm ci
```

### Stopping and resetting

Stop the local containers without deleting the database:

```bash
make stop-local
```

To remove local containers and all volumes belonging to the local Compose project:

```bash
docker compose -f compose.yaml -f compose.local.yaml down -v
```

Warning: this permanently deletes the local SQLite database and other local Docker volumes.

The local secrets directory is not removed by Docker Compose.

### Limitations

The local filesystem storage driver does not currently support generating upload and download URLs required by some media features.

External integrations such as AWS Batch, Telegram, and cloud storage require additional configuration.

The map background may still load from external map tile providers, so this is not a fully offline environment.

## Rust Development

The backend is written in Rust. Rust binaries are slow to compile. It helps to use [sccache](https://crates.io/crates/sccache), especially on build servers like CI/CD.

## Debugging

### Backend Logs

The logs are written to stdout and then handled by Docker depending on how you set it up.

For local setups, you can use `docker logs` to read or tail them. When running in Fly.io, use the [logging console](https://fly.io/apps/treemap-blue-frog-3927/monitoring): it shows recent logs and has a link to a built in Grafana instance which can be used to query logs and search for something specific.
