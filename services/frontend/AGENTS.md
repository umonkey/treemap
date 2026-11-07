# Trees of Yerevan: Frontend

This directory contains the static frontend for the tree mapping application.

## Tech Stack

- Framework: [SvelteKit 5](https://svelte.dev/) (using Runes). Client side rendering only, no SSR at all.
- Language: TypeScript.
- Styling: Custom CSS with variables (see `src/lib/styles`).
- State Management: Svelte Stores and Runes.
- Map Library: MapLibre GL via `svelte-maplibre`.

## Directory Structure

- `src/lib/api`: domain-based API functions.
- `src/lib/components`: domain-based components.
- `src/lib/forms`: complex forms.
- `src/lib/stores`: Svelte stores.
- `src/routes`: SvelteKit pages and layouts, and page-local components.
- `docs/Coding-Style.md`: detailed coding standards.

## Useful commands

- `npm run dev`: start the development server.
- `npm run build`: build the project.
- `npm run check`: run svelte-check for type checking.
- `npm run lint`: run ESLint.
- `npm run test`: run unit tests with Vitest.
- `npm run format`: format code with Prettier.
- `make -C services/frontend format check lint test`: canonical gate; `lint` also runs `prettier --check`.
- `npm run build` is not a local check. Production mode requires `SENTRY_AUTH_TOKEN` and `SENTRY_RELEASE`; `--mode staging` requires every `PUBLIC_*` key in `src/lib/env.ts`. Use `npm run check` for type checking.

## Coding styles

- Use `Form.svelte` to wrap forms.
- Use `Button.svelte` to add buttons to forms.
- Use `Buttons.svelte` to wrap multiple buttons, e.g. in the end of a form.
- Use `TextInput.svelte`, `CheckInput.svelte`, `NumberInput.svelte` and other similar components for forms.
- Use `$app/state` to access page params, do not use `+page.ts`.
- All bigint values returned by the backend need to be converted to strings, as JavaScript cannot handle 64-bit integers.
- API endpoints return `Result<Json<>>` unless they emit no data, then they return a `Result<HttpResponse>` with the right status code.

## Deploy and caching

- `kit.serviceWorker.register` is `false` on purpose. Registration lives in `src/lib/utils/serviceWorker.ts` and treats a refusal as non-fatal. Do not re-enable SvelteKit's auto-registration; it produces unhandled `Rejected` events in Sentry.
- Hashed `_app/immutable` chunks are not retained across deploys. A stale tab fails with `Failed to fetch dynamically imported module`. Treat it as a deployment/staleness signal, not a code bug, and do not blanket-ignore it in Sentry.
