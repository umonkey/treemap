// This store contains markers that should be displayed on the map.
// It serves as a cache when the user goes back to the map view,
// but we didn't yet request any markers.

import { writable } from 'svelte/store';
import type { ITree } from '$lib/types';

export const markerStore = writable<ITree[]>([]);
