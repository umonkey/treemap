import { writable, derived } from 'svelte/store';
import type { IMyPosition } from '$lib/types';

export const locationStore = writable<IMyPosition | null>(null);
export const currentLocation = derived(locationStore, ($location) => $location);
