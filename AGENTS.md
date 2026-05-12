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

When asked to "wrap up", provide a conventional commits styled message for the whole session.


## Documentation

- Check the `docs/` folder for architectural decisions and detailed documentation on specific features.
- See `services/backend/AGENTS.md` and `services/frontend/AGENTS.md` for specific instructions on those subprojects.
- When working on Rust code in the `services/backend` directory, you MUST load the `rust` skill.


## Development Workflow

- When creating plans, arrange them so that each step focuses on one service: frontend, backend, caddy or documentation. This ensures that the domain-specific skills (such as `svelte` or `rust`) are triggered and applied effectively to the relevant sub-tasks.
- Markdown formatting: whenever a Markdown file is added or updated, it must be formatted according to the `markdown` skill. Run `make format-docs` after any changes to files in the `docs/` folder.
- No exploration scripts: never create code files or scripts (e.g., for fetching web pages, querying the database, or system introspection) for the purpose of exploration. Use existing tools (grep, glob, read, bash for direct cli) to gather information.
- Plan mode: when in planning mode, strictly adhere to read-only operations. Do not attempt any file edits, system changes, or commits until explicitly transitioning to build mode.
- When asked to "wrap up", provide a commit message based on the whole session.
- After finishing the changes, suggest a "conventional commits" compatible commit message for the whole session.
