# Svelte 5 Implementation Templates

Use the following structures as your baseline when implementing components and pages.

## 1. Component Logic (`[ComponentName].svelte.ts`)

```typescript
export class ComponentNameLogic {
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
```

## 2. Component Markup (`[ComponentName].svelte`)

```html
<script lang="ts">
  import { ComponentNameLogic } from "./[ComponentName].svelte.ts";

  const componentState = new ComponentNameLogic();
</script>

<div>
  <h1>{componentState.title}</h1>
  <button onclick="{componentState.toggleActive}">
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

## 3. Page Logic (`page.svelte.ts`)

```typescript
export class PageState {
  data = $state<any>(null);

  reload = async (id: string) => {
    // Data fetching logic here
  };
}
```

## 4. Page Markup (`+page.svelte`)

```html
<script lang="ts">
  import { PageState } from "./page.svelte.ts";
  import { page } from "$app/state";

  const pageState = new PageState();

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
