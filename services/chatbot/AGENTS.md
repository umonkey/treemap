# Trees of Yerevan: Chatbot Service

## Scope and Tech Stack

This directory contains the Telegram chatbot service and its background workers.

- Language: Rust
- Framework: [Teloxide](https://teloxide.dev/)
- Database: libSQL / SQLite (shared database with backend)
- Storage: S3 / DigitalOcean Spaces

## Architecture & Directory Structure

- `src/cli/`: command line action commands.
- `src/domains/`: domain models and data repositories.
  - `alert`: citizen feedback report records and eligibility evaluation.
  - `alert_photo`: uploaded photo records associated with alerts.
  - `outbox`: outbox message queue for reliable decoupled Telegram dispatch.
  - `tree`: tree data querying.
- `src/services/`: application business logic and execution loops.
  - `chatbot.rs`: interactive Telegram bot message REPL.
  - `dispatcher.rs`: decoupled background alert dispatcher daemon.
  - `outbox_dispatcher.rs`: outbox message consumer daemon with exponential retries.
  - `i18n.rs`: localization manager using Fluent (`locales/*.ftl`).
- `src/infra/`: infrastructure adapters.
  - `config.rs`: application configuration.
  - `database.rs`: libSQL connection client.
  - `s3.rs`: S3 object storage file client.
  - `secrets.rs`: file-based and environment-based secret loaders.
- `docker/rootfs/etc/supervisor.d/`: production supervisor process configurations (`chatbot.ini`, `dispatch-alerts.ini`, `dispatch-outbox.ini`).

## CLI Commands

The binary (`/app/bin/chatbot`) supports subcommands:

- `serve`: runs the interactive Telegram bot REPL.
- `dispatch-alerts`: runs the background alert dispatcher worker daemon.
- `dispatch-outbox`: runs the outbox queue worker daemon to deliver pending messages.

## Useful Commands

- `make check`: run clippy with `-D warnings` and the test suite.
- `make format`: format Rust code with `cargo fmt`.
- `make serve`: run the chatbot locally.
- `make test`: run unit tests.
