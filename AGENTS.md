# Tree Mapping Application

This folder contains a tree mapping application.
The application is API-first, with a static frontend and an API backend.


## Folder structure

- `backend`: contains the API backend and queue consumers written in Rust and Actix-Web.
- `docs`: contains ADR and other documentation.
- `frontend`: contains the static frontend written with TypeScript and SvelteKit 5, client side rendering only.
- `tools`: contains some additional scripts used for non-regular manual tasks.


## Documentation Style Guidelines

When creating or updating documentation (like ADRs or READMEs), strictly follow these formatting rules:

- List items: always use dashes (`-`) for list items. Do not use asterisks (`*`) or numbers unless explicitly requested.
- Lists: always separate lists from other contents with blank lines.
- Bold text: do not use bold text within sentences or for emphasis. Bold should only be used for structural headers if required by the format.
- Capitalization after colons: do not use upper case letters immediately after a colon (`:`) in a sentence, unless the following word is a proper name or a technical acronym (e.g., "UI", "OSM").
- Sentence starts: always use a capital letter at the beginning of a sentence, including the first word of a list item or a description following a colon-prefixed label.
  - Good: "Database growth: storing full snapshots..."
  - Bad: "Database growth: Storing full snapshots..."
  - Bad: "database growth: storing full snapshots..."


## Development commands

The project uses a `Makefile` for high-level tasks:

- `make build`: build the whole application using Docker.
- `make start`: start the application using Docker Compose.


## Documentation

- Check the `docs/` folder for architectural decisions and detailed documentation on specific features.
- See `backend/AGENTS.md` and `frontend/AGENTS.md` for specific instructions on those subprojects.
