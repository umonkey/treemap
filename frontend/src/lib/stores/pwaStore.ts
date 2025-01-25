import { writable, derived } from 'svelte/store';

export const pwaStore = writable<BeforeInstallPromptEvent | undefined>(undefined);
export const isInstallable = derived(pwaStore, ($pwaStore) => !!$pwaStore);
