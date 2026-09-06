# Tree Mapping Application

This folder contains a tree mapping application. The application is API-first, with a static frontend and an API backend.  The app can be installed as both a PWA, and a TWA via Google Play.


## Folder structure

- `docs`: contains ADR and other documentation.
- `services`: contains the application services.
  - `backend`: contains the API backend and queue consumers written in Rust and Actix-Web.
  - `caddy`: contains the Caddy reverse proxy configuration.
  - `chatbot`: contains the Telegram chatbot and alert dispatcher written in Rust and Teloxide.
  - `extractor`: image and panorama feature extraction service.
  - `frontend`: contains the static frontend written with TypeScript and SvelteKit 5, client side rendering only.
  - `landing`: static landing page.
  - `streetview`: Street View processing pipelines.
- `tools`: contains some additional scripts used for non-regular manual tasks.


## Development commands

The project uses a `Makefile` for high-level tasks:

- `make build`: build the whole application using Docker.
- `make start`: start the application using Docker Compose.


## Database Schema & Migrations

- All SQLite database schemas, seeds, and migration scripts are centralized in `services/backend/dev/`:
  - `schema-sqlite.sql`: Complete current schema (loaded during backend tests and initial setup).
  - `dev/migrations/*.sql`: Sequential migration scripts.
- Do NOT create standalone `.sql` schema files inside other service directories; all services share the database schema defined in `services/backend`.
- When actively developing a feature branch before release, consolidate related schema adjustments into the current in-progress migration rather than generating fragmented individual migration files.


## Background Workers & Daemons

- Services running background tasks (such as `backend` and `chatbot`) use Supervisord in production (`docker/rootfs/etc/supervisor.d/*.ini`).
- When introducing a new background worker, implement it as a CLI subcommand in the service binary and add a corresponding `.ini` file under `docker/rootfs/etc/supervisor.d/`.


## Documentation

- Check the `docs/` folder for architectural decisions and detailed documentation on specific features.
- When working on Rust code in the `services/backend` or `services/chatbot` directory, you MUST load the `rust` skill.


## Development Workflow

- When creating plans, arrange them so that each step focuses on one service: frontend, backend, caddy or documentation. This ensures that the domain-specific skills (such as `svelte` or `rust`) are triggered and applied effectively to the relevant sub-tasks.
- Markdown formatting: whenever a Markdown file is added or updated, it must be formatted according to the `markdown` skill. Run `make format-docs` after any changes to files in the `docs/` folder.
- No exploration scripts: never create code files or scripts (e.g., for fetching web pages, querying the database, or system introspection) for the purpose of exploration. Use existing tools (grep, glob, read, bash for direct cli) to gather information.
- No automatic commits. Never execute git commit, git push or similar version control commands automatically.
- When asked for a solution, suggest the best one for this project, but mention alternatives.
- Strict investigation constraints: focus exclusively on the files named or directly relevant to the user request. Never inspect git history (`git log`, `git show`, `git blame`), never query remote GitHub Actions runs (`gh run`), and never dump or inspect remote CI logs or artifacts unless explicitly and specifically requested to do so. Diagnose configuration and code directly from the workspace files.
