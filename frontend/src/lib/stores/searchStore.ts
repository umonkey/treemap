import { writable } from 'svelte/store';

export const searchStore = writable<string | undefined>(undefined);
