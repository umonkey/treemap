import { writable, derived } from 'svelte/store';
import { DEFAULT_MAP_CENTER } from '$lib/constants';
import { ls } from '$lib/utils/localStorage';

export const mapStore = writable(
	ls.read('mapStore') || {
		center: DEFAULT_MAP_CENTER,
		zoom: 15
	}
);

mapStore.subscribe((value) => {
	ls.write('mapStore', value);
});

export const mapCenter = derived(mapStore, ($mapStore) => $mapStore.center);
export const mapZoom = derived(mapStore, ($mapStore) => $mapStore.zoom);
