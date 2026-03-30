import { DEFAULT_MAP_CENTER } from '$lib/constants';
import type { ILatLng } from '$lib/types';
import { ls } from '$lib/utils/localStorage';
import { derived, writable } from 'svelte/store';

const STORE_ID = 'mapStore-v3';

type IMapStore = {
	center: ILatLng;
	zoom: number;
};

export const mapStore = writable<IMapStore>(
	ls.read(STORE_ID) || {
		center: DEFAULT_MAP_CENTER,
		zoom: 15
	}
);

mapStore.subscribe((value) => {
	// console.debug('[map] Storing center and zoom.');
	ls.write(STORE_ID, value);
});

export const mapCenter = derived(mapStore, ($mapStore) => $mapStore.center);
export const mapZoom = derived(mapStore, ($mapStore) => $mapStore.zoom);
