// This store controls the tree preview sidebar on the map.
// It contains the tree id to display, or undefined to hide.
// It is manipulated by the Map component, and used by the MapPreview component.

import { writable } from 'svelte/store';

export const mapPreviewStore = writable<string | undefined>(undefined);
