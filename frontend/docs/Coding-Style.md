# Coding Style

## General rules

- Adhere strictly to idiomatic SvelteKit 5:
  - Runes: use `$state`, `$derived`, `$effect` and `$props` exclusively. Do not use legacy reactivity (`$:`, `export let`).
  - Snippets over slots: use snippets (`{#snippet}`) and `{@render ...}` instead of the legacy `<slot>` system.
  - Callback props over dispatch: use callback props (e.g., `onSave()`) instead of `createEventDispatcher`.
- Prop declaration:
  - Default to `const`: use `const { ... } = $props()` for standard, read-only data flow.
  - Use `let` only for binding: only switch to `let { ... } = $props()` if the component requires at least one `$bindable()` prop.
- Do not make big pages. Pages should only orchestrate components.

## Page structure

All pages are wrapped with the `NarrowPage` component, and using the `Header` component to display a title. Example:

```svelte
import {(Header, NarrowPage)} from '$lib/ui';

<svelte:head>
	<title>Mapper mode</title>
</svelte:head>

<Header title="Mapper mode" />

<NarrowPage>
	<!-- page contents goes here -->
</NarrowPage>
```

## Component structure

Components live in the `src/lib/components` folder, each component in its own folder.

The markup of the component stays in the `.svelte` file, the styles go the `.css` file, and the TypeScript code goes to the `.ts` file.

## Error handling

For reporting errors from components, use the `toast` library, like this:

```typescript
import { toast } from '@zerodevx/svelte-toast';
toast.push('Something bad happened.);
```
