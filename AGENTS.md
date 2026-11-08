# Tree Mapping Application

API-first tree mapping platform: static SvelteKit frontend, Actix-Web Rust backend, Teloxide chatbot, and Python feature extraction pipelines.

## Repository Structure & Sub-Service Documentation

- `docs/`: Architectural decisions (ADRs) and project documentation.
- `services/backend/`: Rust Actix-Web API (`services/backend/AGENTS.md`).
- `services/chatbot/`: Rust Teloxide bot & alert daemons (`services/chatbot/AGENTS.md`).
- `services/frontend/`: TypeScript & SvelteKit 5 static client (`services/frontend/AGENTS.md`).
- `services/extractor/`: Python SfM & 360 panorama processing (`services/extractor/AGENTS.md`).
- `services/caddy/`: Caddy reverse proxy.
- `tools/`: Standalone administrative CLI utilities.

## Skills

Before modifying or creating files, load every skill whose `description` matches the task. Repository skill descriptions are auto-injected by OpenCode from `.agents/skills/*/SKILL.md` and are the authoritative triggers; do not maintain a separate routing table here. When scopes overlap, load both (for example `feature-docs` for a capability page, then `markdown` for formatting).

## Core Operational Constraints

- Plan granularity: partition by file ownership and service boundary so each scope triggers the right skill, but name scopes by domain (`upload`, `map`, `alerts`, `osm`), never by layer (`frontend`, `backend`, `docs`).
- Implementation first: NEVER run tests, linters, or build checks before implementing required code.
- Investigation constraints: inspect only directly relevant files. Never inspect git history (`git log`, `git blame`), CI runs (`gh run`), or remote artifacts unless explicitly commanded.
- Repository-wide searches: never run a recursive `grep` from the repository root without `--exclude-dir=target`. `services/*/target` holds tens of GB and the command times out; scope to `src` instead.
- No exploration scripts: do not generate throwaway scripts or code files to probe the system. Use existing tools (`grep`, `glob`, `read`, direct CLI).
- No automatic commits: never run `git commit` or `git push` autonomously.
- No lint suppressions: never add `-A`/`--allow` lint flags or new `#[allow]` attributes to make `make check` pass. Fix the code or align the toolchain; the gate must stay green without suppressions.
- Container limitations: Docker daemon is unavailable inside this container. Do NOT run `docker` or `docker compose` commands.
- Formatting: terminal-friendly ASCII/Unicode only (no LaTeX formulas). Run `make format-docs` after modifying any file in `docs/`.
- ADRs: only for a contract shared across features or services, or an irreversible trade-off. Do not add one for an isolated tool, an additive feature, or routine work. Follow `.agents/skills/adr/SKILL.md` and ask the operator when unsure.

## Build Gotchas

- The cron-daily that ships is services/backend/docker/rootfs/app/bin/cron-daily. container/rootfs has no scheduler, so add backend cron jobs there. The legacy container/Dockerfile.yerevan image copies only container/rootfs and will not run them.
- make format-docs formats every Markdown file under docs/ and .agents/, so run it after editing a skill, not only after editing docs.
- Rust is pinned to 1.97 by the root `rust-toolchain.toml`, matching the `rust:1.97-bookworm` image tag and both `rust-version` fields. Bumping the channel changes Clippy's findings (1.99 flags `double_must_use` on `#[async_trait]` methods returning `Result`; 1.97 does not) and invalidates every build artifact, which Cargo never deletes. Run `cargo clean` in the affected service before switching, and answer a new lint by fixing the code or aligning the toolchain, never by adding an allow.
- `cargo clippy` does not lint `#[cfg(test)]` code, so the crate-wide `unwrap_used = "deny"` never reaches test modules and an allow there suppresses nothing. Do NOT add `--all-targets` to check this: it applies the deny to tests and fails for reasons unrelated to the change.

## Verification

Run the canonical gate for each changed service from the repository root:

- Backend: `make -C services/backend check`.
- Chatbot: `make -C services/chatbot check`.
- Frontend: `make -C services/frontend format check lint test`.
- Docs: `make format-docs`.

Both backend and frontend test targets accept `FILTER=<pattern>` for a focused run.

After changing Rust, confirm no suppression survived: `grep -rn --include='*.rs' 'clippy::' services/backend/src services/chatbot/src` must print nothing, and no `-A clippy` may appear in a Makefile or workflow.

## Commit Messages

Use Conventional Commits. The scope is the domain (`upload`, `map`, `alerts`, `osm`, `segmentation`), never the layer (`frontend`, `backend`, `docs`). The body is a short bullet list of user-visible changes. Do not quote or backtick symbol names. Mark API or migration breaks with `BREAKING CHANGE:`.

## Architectural Invariants

- The backend and chatbot share one SQLite database. A schema change to a table both services read (`chatbot_alerts`, `chatbot_alerts_photos`, `trees`, `instances`) must update both services in the same change.
- Backend CLI commands build state with AppState::new() and must not call session() when the domain opens its own transaction. session() starts a transaction, and a nested Database::transact() fails with "Already in a transaction"; use it only for tests or single-transaction flows.
- storage-sync holds one whole bucket in memory because list_files returns a Vec. Do not point it at buckets with millions of objects without first streaming listing pages into the upsert. Query live usage with missing_since IS NULL.
