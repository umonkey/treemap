import { writable, derived } from 'svelte/store';
import type { IMyLocation } from '$lib/types';

export const locationStore = writable<IMyLocation | null>(null);
export const currentLocation = derived(locationStore, ($location) => $location);
