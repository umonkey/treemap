import { writable } from 'svelte/store';
import { mapBus } from '$lib/buses/mapBus';

export const searchStore = writable<string | undefined>(undefined);

searchStore.subscribe((value) => {
	console.debug(`[search] Search store updated: ${value}`);
	mapBus.emit('reload');
});
