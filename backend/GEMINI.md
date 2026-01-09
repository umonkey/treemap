# Trees of Yerevan: Backend

## Scope and Tech Stack

This directory contains the API, queue consumers and other backend code.

- The code is written in Rust.
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

- No automatic commits. Never execute git commit, git push or similar version control commands automatically.
- When asked for a solution, suggest the best one for this project, but mention alternatives.
- Before implementing any changes, provide the implementation plan and ask for the user's confirmation.
- After creating the code, run "cargo fmt" to fix any formatting issues, then "make lint" to make sure all code is correct. Fix any reported issues.
