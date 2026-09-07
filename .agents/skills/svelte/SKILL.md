---
name: svelte
description: Svelte 5 runes and separated-logic class architecture for services/frontend/. Use when creating, modifying, or refactoring *.svelte and *.svelte.ts files.
---

# Svelte 5 Architecture Directives

Strictly adhere to the following rules when creating or modifying Svelte components or pages in `services/frontend/`.

## Core Invariants

1. Client-Only: CSR only (no SSR). Forbidden: `+page.ts` or SvelteKit `load` functions. All fetching occurs in the logic class.
2. File Separation: exactly two files per entity:
   - Components: `[Name].svelte` (markup) and `[Name].svelte.ts` (logic class).
   - Pages: `+page.svelte` (markup) and `page.svelte.ts` (logic class).
3. Class-Based Logic: state and business logic must reside in a TypeScript class using Svelte 5 `$state` runes.
4. Method Signatures: all class methods MUST be arrow functions to preserve lexical `this`.
5. Pure Constructors: constructors MUST NOT execute side effects, subscriptions, or API calls. Fetch via `reload()` or `init()` invoked from an `$effect` block; use `onMount()` for mounting logic.
6. Instantiation: export classes as named exports. Instantiate locally in `<script lang="ts">`:
   - Components: `const componentState = new ComponentNameLogic();`
   - Pages: `const pageState = new PageState();`
7. Placement:
   - Page-local: single-page components stay in that route directory.
   - Shared: multi-page components reside in `src/lib/components/<domain>/`.
8. Styles: embedded `<style>` blocks only. No standalone `.css` files. Use PicoCSS variables first.
9. Runes Only: use `$state`, `$derived`, `$effect`, and `$props` exclusively. No Svelte 4 legacy slots or event dispatchers.
10. Prop Typing: strictly type destructuring: `const { prop }: { prop: Type } = $props()`. Generic syntax `$props<Type>()` is forbidden. Default to `const`; use `let` only for `$bindable()`.
11. Pages: wrap contents in `<RoleGuard roles={['Scientist']}>` unless specified otherwise.

## Implementation Templates & Reference

For full boilerplate implementations (Component Logic, Component Markup, Page Logic, Page Markup), read:
`references/templates.md` (located inside this skill folder).

## Verification Workflow

After modifying components or pages, verify within `services/frontend/`:

```bash
make -C services/frontend format check lint
```
