# Tree Mapping Application

This folder contains a tree mapping application.
The application is API-first, with a static frontend and an API backend.


## Folder structure

- `backend`: contains the API backend and queue consumers written in Rust and Actix-Web.
- `docs`: contains ADR and other documentation.
- `frontend`: contains the static frontend written with TypeScript and SvelteKit 5.
- `tools`: contains some additional scripts used for non-regular manual tasks.


## Development commands

The project uses a `Makefile` for high-level tasks:

- `make build`: build the whole application using Docker.
- `make start`: start the application using Docker Compose.


## Documentation

- Check the `docs/` folder for architectural decisions and detailed documentation on specific features.
- See `backend/AGENTS.md` and `frontend/AGENTS.md` for specific instructions on those subprojects.
