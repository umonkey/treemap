import { writable } from 'svelte/store';

export const searchStore = writable<string | undefined>(undefined);

searchStore.subscribe((value) => {
	console.debug(`[search] Search store updated: ${value}`);
});
