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

## Skill Intent Routing

Before modifying or creating files, you MUST load the appropriate skill via the `skill` tool:

| Intent / Task Target | Required Skill | Boundary Trigger / Paths |
| :--- | :--- | :--- |
| Svelte components, pages, or frontend logic | `svelte` | `services/frontend/src/**`, `*.svelte`, `*.svelte.ts` |
| Rust backend API or chatbot development | `rust` | `services/backend/**`, `services/chatbot/**`, `*.rs` |
| Extending the backend JSON-RPC MCP server | `mcp-server` | `services/backend/src/services/mcp/**` |
| Creating or reporting GitHub repository issues | `github-issues` | Invoking `gh issue create`, reporting bugs/tasks |
| Modifying markdown documentation files | `markdown` | `docs/**/*.md` |

## Core Operational Constraints

- Plan granularity: organize multi-step plans by individual service (frontend, backend, caddy, docs) to trigger relevant skills cleanly.
- Implementation first: NEVER run tests, linters, or build checks before implementing required code.
- Investigation constraints: inspect only directly relevant files. Never inspect git history (`git log`, `git blame`), CI runs (`gh run`), or remote artifacts unless explicitly commanded.
- No exploration scripts: do not generate throwaway scripts or code files to probe the system. Use existing tools (`grep`, `glob`, `read`, direct CLI).
- No automatic commits: never run `git commit` or `git push` autonomously.
- Container limitations: Docker daemon is unavailable inside this container. Do NOT run `docker` or `docker compose` commands.
- Formatting: terminal-friendly ASCII/Unicode only (no LaTeX formulas). Run `make format-docs` after modifying any file in `docs/`.
