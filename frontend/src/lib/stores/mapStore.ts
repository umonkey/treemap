import { DEFAULT_MAP_CENTER } from '$lib/constants';
import { ls } from '$lib/utils/localStorage';
import { derived, writable } from 'svelte/store';

type IMapStore = {
	center: number[];
	zoom: number;
	last: string | null;
};

export const mapStore = writable<IMapStore>(
	ls.read('mapStore') || {
		center: DEFAULT_MAP_CENTER,
		zoom: 15,
		last: null
	}
);

mapStore.subscribe((value) => {
	console.debug('[map] Storing center and zoom.');
	ls.write('mapStore', value);
});

export const mapCenter = derived(mapStore, ($mapStore) => $mapStore.center);
export const mapZoom = derived(mapStore, ($mapStore) => $mapStore.zoom);
export const mapLastTree = derived(mapStore, ($mapStore) => $mapStore.last);

export const setLastTree = (last: string | null) => {
	mapStore.update((state) => {
		return { ...state, last: last };
	});

	console.debug(`[map] Last active tree set to ${last}`);
};
