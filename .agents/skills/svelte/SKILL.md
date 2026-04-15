---
name: svelte
description: Generates Svelte 5 components using a strict separated-logic pattern. Use this whenever creating, extending, or refactoring a Svelte component or page.
---

## Core Architecture Directives

When generating or modifying a Svelte component or page, you MUST adhere strictly to the following rules. Do not use generic Svelte 4 or standard Svelte 5 script tag logic.

1. Client-Only Architecture: the application is client-side only (no SSR). Avoid using `+page.ts` or SvelteKit's standard `load` functions. All data fetching must happen within the logic class.
2. File separation: every component or page MUST be split into exactly two files:
   - For components: the markup file (`[ComponentName].svelte`) and the logic file (`[ComponentName].svelte.ts`).
   - For pages: the markup file (`+page.svelte`) and the logic file (`page.svelte.ts`).
3. Class-based logic: all reactive state and business logic must be encapsulated within a TypeScript class in the `.svelte.ts` file.
4. State management: use the Svelte 5 `$state` rune for all reactive properties inside the class.
5. Method signatures: all class methods MUST be defined using arrow functions to permanently bind `this` and preserve lexical scope.
6. Pure constructors: the constructor must be strictly pure. It is entirely forbidden to execute side effects, API calls, or subscriptions within the constructor.
   - If the component/page needs any data, use a dedicated `reload` or `init` method triggered via `$effect`.
   - If the component/page needs code executed on mount, create the `onMount` method.
7. Singleton export: the class must be instantiated at the bottom of the `.svelte.ts` file and exported as a singleton.
   - For components: strictly named `componentState`.
   - For pages: strictly named `pageState`.
8. Component placement and routing:
   - Default (page-local): if a component is only utilized by a single page, you MUST co-locate the component files directly within that specific route's directory.
   - Shared (domain): if a component is utilized by more than one page, you MUST place the component files in the `src/lib/components/<domain>/` directory.
   - Pages: these rules do not apply to pages; they always reside in their respective route directory.
   - Refactoring: if you are instructed to use a page-local component on a second page, you must first move its files to `src/lib/components/<domain>/` and update all existing import paths before proceeding.
9. Style Directives:
   - Embedded Styles Only: you MUST use the Svelte `<style>` block within the `.svelte` file for all component-specific or page-specific styling. Creating separate `.css` files per component is strictly forbidden.
   - PicoCSS First: rely on PicoCSS for layout, typography, and basic component visuals. Avoid manual styling for standard elements (buttons, inputs, cards, etc.) unless specifically requested or required for custom behavior.
10. Adhere strictly to idiomatic Svelte 5:
    - Use runes: use `$state`, `$derived`, `$effect`, and `$props` exclusively.
    - Snippets over slots: use Snippets (`{#snippet}`) and `{@render ...}` instead of the legacy `<slot>` system.
    - Callback props over dispatch: use callback props (e.g., `onSave()`) instead of `createEventDispatcher`.
11. Properties declaration (components only):
    - Explicit Typing Required: you MUST use explicit typing on the destructuring assignment (e.g., `const { prop }: { prop: Type } = $props()`). Do NOT use the generic syntax `$props<Type>()` as it currently fails to correctly propagate type information to the linter.
    - Default to `const`: use `const { ... }: { ... } = $props()` for standard, read-only data flow.
    - Use `let` only for Binding: only switch to `let { ... }: { ... } = $props()` if the component requires at least one `$bindable()` prop.
12. Verification Workflow: after implementing or modifying components/pages, you MUST verify the changes by running `make format` and `make lint check` within the `services/frontend` directory.


## Page Architecture

1. All new pages must use a `RoleGuard` to wrap their contents.  Unless specified otherwise, roles should be set to `['Scientist']`.


## Implementation Template

You must use the following structure as your baseline.

### 1. Component Logic (`[ComponentName].svelte.ts`)

```typescript
class ComponentNameLogic {
    // 1. Reactive state using runes
    title = $state<string>("Default Title");
    isActive = $state<boolean>(false);

    constructor() {
        // 2. Pure constructor: NO side effects allowed here.
    }

    // 3. Methods as arrow functions
    toggleActive = () => {
        this.isActive = !this.isActive;
    };
}

export const componentState = new ComponentNameLogic();
```

### 2. Component Markup (`[ComponentName].svelte`)

```html
<script lang="ts">
    import { componentState } from './[ComponentName].svelte.ts';
</script>

<div>
    <h1>{componentState.title}</h1>
    <button onclick={componentState.toggleActive}>
        {componentState.isActive ? 'Deactivate' : 'Activate'}
    </button>
</div>

<style>
    /* 4. Embedded styles only, rely on PicoCSS for basics */
    div {
        margin-top: var(--pico-spacing);
    }
</style>
```

### 3. Page Logic (`page.svelte.ts`)

```typescript
class PageState {
    data = $state<any>(null);

    reload = async (id: string) => {
        // Data fetching logic here
    };
}

export const pageState = new PageState();
```

### 4. Page Markup (`+page.svelte`)

```html
<script lang="ts">
    import { pageState } from './page.svelte.ts';
    import { page } from '$app/state';

    // Reactive extraction of non-optional arguments
    const id = $derived(page.params.id as string);

    $effect(() => {
        // Trigger reload when ID changes
        pageState.reload(id);
    });
</script>

<article>
    <header>Page Title</header>
    {#if pageState.data}
        <pre>{JSON.stringify(pageState.data, null, 2)}</pre>
    {:else}
        <p aria-busy="true">Loading...</p>
    {/if}
</article>
```
