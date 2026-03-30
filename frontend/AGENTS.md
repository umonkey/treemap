# Trees of Yerevan: Frontend

This directory contains the static frontend for the tree mapping application.

## Tech Stack

- **Framework**: [SvelteKit 5](https://svelte.dev/) (using Runes), client side rendering only.
- **Language**: TypeScript.
- **Styling**: Custom CSS with variables (see `src/lib/styles`).
- **State Management**: Svelte Stores and Runes.
- **Map Library**: MapLibre GL via `svelte-maplibre`.

## Directory Structure

- `src/lib/components`: domain-based components.
- `src/lib/forms`: complex forms.
- `src/lib/stores`: Svelte stores.
- `src/routes`: SvelteKit pages and layouts.
- `docs/Coding-Style.md`: detailed coding standards.

## Development workflow

- Strictly follow the coding style described in [docs/Coding-Style.md](docs/Coding-Style.md).
- Use Svelte 5 Runes ($state, $derived, $effect, $props) for new components.
- Note: Some existing components might still use legacy Svelte syntax (e.g., `export let`); update them to Runes when possible.
- After changing code, run `npm run check` and `npm run lint`.

## Code Formatting

- Use single quotes when possible.
- Try to maintain the existing formatting style, avoid re-formatting the code unless explicitly asked to.

## Useful commands

- `npm run dev`: start the development server.
- `npm run build`: build the project.
- `npm run check`: run svelte-check for type checking.
- `npm run lint`: run ESLint.
- `npm run test`: run unit tests with Vitest.
- `npm run format`: format code with Prettier.
