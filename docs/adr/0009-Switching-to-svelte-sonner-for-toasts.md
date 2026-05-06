# ADR-0009: Switching to svelte-sonner for Toasts

- Date: 2026-05-06
- Status: Accepted

## Context

The application previously used `@zerodevx/svelte-toast` for displaying temporary messages and error notifications. While lightweight, this library lacked native support for click handlers and interactive actions on toasts. Implementing features like "click to view details" required complex workarounds or custom Svelte components for every instance.

As the application matures, we require a more robust toast system that supports:

- Native click actions: easily linking a toast to a navigation event or callback.
- Stacking visuals: modern "stacked" toast appearance that doesn't clutter the viewport.
- Rich colors and icons: better visual distinction between error, success, and info states.
- Close buttons: explicit dismissability for all toasts.

## Decision

We will replace `@zerodevx/svelte-toast` with `svelte-sonner`.

- Implementation: the `SvelteToast` component in `+layout.svelte` is replaced with `Toaster`.
- Configuration: toasts are positioned at `bottom-left` to avoid overlapping with primary navigation elements.
- Utility: the central `showError` utility in `src/lib/errors.ts` is updated to use `toast.error()`.
- Default features: `richColors` and `closeButton` are enabled by default for all notifications.

## Consequences

Positive:

- Improved UX: users can now interact with toasts via explicit actions or by clicking the message.
- Developer experience: `svelte-sonner` provides a cleaner API for handling complex notification flows (like promise-based loading states).
- Visual consistency: the stacking behavior provides a more polished, modern feel.

Negative:

- Dependency change: requires managing a new third-party dependency.
- Migration effort: any direct usage of the old `toast` store must be updated to the new API.
