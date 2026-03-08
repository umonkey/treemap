import { DEFAULT_MAP_CENTER } from '$lib/constants';
import { ls } from '$lib/utils/localStorage';
import { derived, writable } from 'svelte/store';
import type { ILatLng } from '$lib/types';

const STORE_ID = 'mapStore-v3';

type IMapStore = {
	center: ILatLng;
	zoom: number;
};

export const mapStore = writable<IMapStore>(
	ls.read(STORE_ID) || {
		center: {
			lat: DEFAULT_MAP_CENTER[0],
			lng: DEFAULT_MAP_CENTER[1]
		},
		zoom: 15
	}
);

mapStore.subscribe((value) => {
	console.debug('[map] Storing center and zoom.');
	ls.write(STORE_ID, value);
});

export const mapCenter = derived(mapStore, ($mapStore) => $mapStore.center);
export const mapZoom = derived(mapStore, ($mapStore) => $mapStore.zoom);
