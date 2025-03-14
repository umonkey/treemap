import { derived, writable } from "svelte/store";

// @ts-expect-error 2304
export const pwaStore = writable<BeforeInstallPromptEvent | undefined>(
	undefined,
);
export const isInstallable = derived(pwaStore, ($pwaStore) => !!$pwaStore);
