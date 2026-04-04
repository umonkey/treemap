# Coding Style

## Folder Structure

- `src/lib/buses`: contains mitt based buses to send signals across components.
- `src/lib/components`: contains components, grouped in domain-based subfolders, e.g. "comments", "observations".
- `src/lib/forms`: contains forms that consist of multiple components.
- `src/lib/icons`: contains SVG icons renamed as Svelte components for inlining.
- `src/lib/stores`: contains Svelte stores.
- `src/lib/styles`: contains global style sheets.
- `src/lib/ui`: contains simple components, deprecated over `src/lib/components`.
- `src/lib/utils`: contains helper functions that aren't related to components or Svelte.
- `src/lib/api/`: domain-based API functions.
- `src/lib/locale.ts`: UI translation strings for English, Russian and Armenian languages.
- `src/lib/routes.ts`: contains functions that format page paths, serves as a single source of truth.

## General rules

- Adhere strictly to idiomatic SvelteKit 5:
  - Runes: use `$state`, `$derived`, `$effect` and `$props` exclusively. Do not use legacy reactivity (`$:`, `export let`).
  - Snippets over slots: use snippets (`{#snippet}`) and `{@render ...}` instead of the legacy `<slot>` system.
  - Callback props over dispatch: use callback props (e.g., `onSave()`) instead of `createEventDispatcher`.
- API Interaction:
  - Do not use a single API client class.
  - Split API calls into domain-specific files in `src/lib/api/`.
  - Use the shared `request` and `getAuthHeaders` from `src/lib/api/client.ts`.
  - Import API functions directly to enable efficient tree-shaking and chunking.
  - Avoid creating a central `index.ts` barrel file for the API.
- Prop declaration:
  - Default to `const`: use `const { ... } = $props()` for standard, read-only data flow.
  - Use `let` only for binding: only switch to `let { ... } = $props()` if the component requires at least one `$bindable()` prop.
- Do not make big pages. Pages should only orchestrate components.

## Page structure

All pages are wrapped with the `Dialog` component. Example:

```svelte
import Dialog from '$lib/components/layout/Dialog.svelte';

<Dialog title="Mapper mode">
	<!-- page contents goes here -->
</Dialog>
```

## Component structure

Components live in the `src/lib/components` folder, each component in its own folder.

The markup of the component stays in the `.svelte` file, the styles are embedded in the component file, and the TypeScript code goes to the `.ts` file.

Components are being imported from their own `.svelte` files. Avoid using the deprecated `src/lib/components/index.ts`, as it breaks the chunking.

## Error handling

For reporting errors from components, use the `toast` library, like this:

```typescript
import { toast } from '@zerodevx/svelte-toast';
toast.push('Something bad happened.);
```
