# Tree Mapping Application

This folder contains a tree mapping application. The application is API-first, with a static frontend and an API backend.  The app can be installed as both a PWA, and a TWA via Google Play.


## Folder structure

- `docs`: contains ADR and other documentation.
- `services`: contains the application services.
  - `backend`: contains the API backend and queue consumers written in Rust and Actix-Web.
  - `caddy`: contains the Caddy reverse proxy configuration.
  - `frontend`: contains the static frontend written with TypeScript and SvelteKit 5, client side rendering only.
- `tools`: contains some additional scripts used for non-regular manual tasks.


## Development commands

The project uses a `Makefile` for high-level tasks:

- `make build`: build the whole application using Docker.
- `make start`: start the application using Docker Compose.


## Documentation

- Check the `docs/` folder for architectural decisions and detailed documentation on specific features.
- See `services/backend/AGENTS.md` and `services/frontend/AGENTS.md` for specific instructions on those subprojects.
- When working on Rust code in the `services/backend` directory, you MUST load the `rust` skill.


## Development Workflow

- When creating plans, arrange them so that each step focuses on one service: frontend, backend, caddy or documentation. This ensures that the domain-specific skills (such as `svelte` or `rust`) are triggered and applied effectively to the relevant sub-tasks.
- Markdown formatting: whenever a Markdown file is added or updated, it must be formatted according to the `markdown` skill. Run `make format-docs` after any changes to files in the `docs/` folder.
- No exploration scripts: never create code files or scripts (e.g., for fetching web pages, querying the database, or system introspection) for the purpose of exploration. Use existing tools (grep, glob, read, bash for direct cli) to gather information.
- Phase 1: Planning (Main Session): The main session is for exploration and planning only. Maintain a strictly read-only approach: do not edit files, change configurations, or make commits. Use this phase to understand requirements and provide a detailed plan for user approval.
- Phase 2: Execution (Sub-Agent): Once the plan is approved and you receive an explicit request to execute (e.g., "apply the plan"), delegate ALL implementation tasks to the `@implement` sub-agent. The main session must never perform file modifications or system changes.
- Verification: The `@implement` sub-agent must verify all changes by running `make format check` (or the service-specific equivalent) before concluding its task.
- No automatic commits. Never execute git commit, git push or similar version control commands automatically.
- When asked for a solution, suggest the best one for this project, but mention alternatives.
