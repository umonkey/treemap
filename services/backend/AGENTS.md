# Trees of Yerevan: Backend

## Scope and Tech Stack

This directory contains the API, queue consumers and other backend code.

- The code is written in Rust 1.95.
- The web framework is Actix-Web.
- The database engine is SQLite.

## Architectural patterns

The API follows the Action-Domain-Responder model.

## Directory Structure

- src/actions: web actions. These extract input data from the request, pass it to the domain implementation, then convert the results back to the API response.
- src/cli: command line actions. These extract input data from the CLI arguments, pass it to the domain implementation, then present the results.
- src/domain: this is where the business logic lives, organized by domain.
- src/infra: infrastructure adapters. The code that talks to external services, like db, queue, file storage. Should not be interdependent, unless absolutely necessary.
- src/services: additional application logic which orchestrates domains. Does not have any domain specific knowledge.
- src/utils: common simple things with no dependencies.

## Development workflow

- Load the `rust` skill using the `skill` tool before starting any work on the backend code.
- No automatic commits. Never execute git commit, git push or similar version control commands automatically.
- When asked for a solution, suggest the best one for this project, but mention alternatives.
- Before implementing any changes, provide the implementation plan and ask for the user's confirmation.

## Useful commands

- `make build`: build the project (release mode).
- `make check`: run clippy and check the code.
- `make format`: format the code.
- `make serve`: run the backend locally.
- `make test`: run unit tests.
- `make sqlite-seed`: install test data into a local SQLite database.

### CLI Commands

The binary also supports several CLI commands (run via `cargo run -- <command>`):

- `osm-pull`: get new trees from OpenStreetMap.
- `osm-push`: send new trees to OSM.
- `osm-push-changes`: send tree updates to OSM.
- `queue-consumer`: run the queue consumer daemon.
- `serve`: run the web server.
- `update-tree-address N`: update street address for a single tree.
- `update-tree-addresses`: update street address for all trees.
- `upload-files`: move local files to S3.
