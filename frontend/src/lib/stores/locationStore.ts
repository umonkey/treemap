// This store contains last received geolocation coordinates.
// It is used to display the user's current position on the map,
// and to pan the map to the user's position when requested by the MapLocateMe control.

import type { IMyPosition } from '$lib/types';
import { derived, writable } from 'svelte/store';

export const locationStore = writable<IMyPosition | null>(null);
export const currentLocation = derived(locationStore, ($location) => $location);
