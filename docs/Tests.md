# Testing TreeMap

TreeMap has separate tests for the Rust backend, the Svelte frontend, and the local Docker Compose setup. You can run them independently, depending on what you changed.

## Backend tests

From the repository root, run:

```sh
make -C services/backend test
```

This runs the Rust tests. It requires a local Rust toolchain and the backend's build dependencies. These tests do not require the local Docker Compose environment to be running.

## Frontend tests

From the repository root, run:

```sh
cd services/frontend
npm ci
npm test
```

The frontend uses Vitest. `npm ci` installs the dependencies defined by the lockfile; you usually only need to repeat it when those dependencies change. The tests do not require the TreeMap backend to be running.

## Testing the local development environment

The local environment has its own checks in `tests/local/test_local.py`. They are designed to catch configuration mistakes before those mistakes become hard-to-debug Docker startup errors.

You need **Python 3.9 or newer**. The tests use only Python's standard library (including on macOS with Python 3.9), so you do not need to install any Python packages. Docker Compose v2 is needed for the Compose checks; the running-container tests also need the local environment to be up.

### First, check the configuration

From the **root of the TreeMap repository**, run:

```sh
sh tools/init-local.sh
python3 -m unittest discover -s tests/local -p 'test_*.py' -v
```

This verifies that:

- `config.toml.local` selects SQLite and local file storage, without an SQS URL.
- Cron is disabled in `local` but remains enabled in the original `dev` configuration.
- The local email dispatcher does not try to rotate Docker output streams.
- The database bootstrap creates the required instance and permissions, can be run twice, and does not erase trees already in the database.
- The local startup script has valid shell syntax and the secrets initializer can safely run more than once.
- The merged Docker Compose configuration selects the local backend, mounts the necessary files, and points every Compose secret at `.local-secrets/`.

The SQL test uses an **in-memory SQLite database**. It does not open or modify the SQLite database in your Docker volume. These checks do not start or stop containers. Tests requiring a running application are shown as *skipped* during this first run; that is expected.

### Then, test the running application

Start the local environment first, for example in detached mode:

```sh
docker compose -f compose.yaml -f compose.local.yaml up -d --build
```

In the repository root, run:

```sh
TREEMAP_SMOKE=1 python3 -m unittest discover -s tests/local -p 'test_*.py' -v
```

In addition to the configuration checks, this tests that the backend and frontend containers are running, the backend `/health` endpoint responds, the frontend serves HTML, the API returns the local instance settings, cron is stopped, the backend web process is running, the backend uses `config.toml.local`, the local instance exists in SQLite, and Vite is installed inside the frontend container.

The smoke tests **only read** from the running services and database. They do not reset the database, delete volumes, import trees, or contact AWS or OpenStreetMap. The tests also do not require the tree count to be zero: you may already have added trees yourself.

### Understanding the result

- **OK** means the assertions passed.
- **Skipped** is normal for the smoke tests when `TREEMAP_SMOKE` is not set, or for Compose checks if Docker is unavailable.
- **FAIL** means an assertion did not match what was expected.
- **ERROR** means a command failed, a required file is missing, or the local service did not respond. Check the error text and container logs.

A Docker container can be running while its web process is not. If the frontend responds but `localhost:8000` refuses connections, first check the backend processes:

```sh
docker compose -f compose.yaml -f compose.local.yaml ps -a
docker compose -f compose.yaml -f compose.local.yaml exec -T backend supervisorctl status
curl -i http://localhost:8000/health
```

In the Supervisor output, `web` should be **RUNNING** and `crond` should be **STOPPED**. A nonzero exit status from `supervisorctl status` can be normal when asking about a deliberately stopped process; the tests check the status text rather than requiring exit code zero. If `web` is not running, inspect the backend logs below.

The SQLite test uses `sqlite3 -batch -noheader -list` so it works even if the container has a `.sqliterc` that formats query results as a table.

To see the local service logs:

```sh
docker compose -f compose.yaml -f compose.local.yaml logs --tail=100 backend frontend
```

Both the original `dev` and the new `local` environment publish ports `8000` and `5173` by default. Stop the original `dev` before starting `local`, or Docker will report a port conflict. Stopping `dev` with `docker compose -f compose.yaml down` leaves its database volume intact.

### Optional Makefile shortcuts

If you prefer shorter commands, add the following targets to the **root** `Makefile`. The test archive does not replace your existing `Makefile`.

```makefile
# Validate the local configuration without starting containers.
.PHONY: test-local
test-local: local-init
	python3 -m unittest discover -s tests/local -p 'test_*.py' -v

# Run configuration and read-only checks against running local containers.
.PHONY: test-local-smoke
test-local-smoke: local-init
	TREEMAP_SMOKE=1 python3 -m unittest discover -s tests/local -p 'test_*.py' -v
```

Then use `make test-local` before startup and `make test-local-smoke` after startup. These targets assume that your local setup already defines the `local-init` target.
