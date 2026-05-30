import { DEFAULT_MAP_CENTER } from '$lib/constants';
import { showError } from '$lib/errors';
import { locale } from '$lib/locale';
import type { ILatLng } from '$lib/types';
import { ls } from '$lib/utils/localStorage';
import { derived, writable } from 'svelte/store';

const STORE_ID = 'mapStore-v4';

type IMapStore = {
	center: ILatLng;
	zoom: number;
	bearing: number;
};

export const mapStore = writable<IMapStore>(
	ls.read(STORE_ID) || {
		center: DEFAULT_MAP_CENTER,
		zoom: 15,
		bearing: 0
	}
);

mapStore.subscribe((value) => {
	// console.debug('[map] Storing center and zoom.');
	if (!ls.write(STORE_ID, value)) {
		showError(locale.toastStorageError());
	}
});

export const mapCenter = derived(mapStore, ($mapStore) => $mapStore.center);
export const mapZoom = derived(mapStore, ($mapStore) => $mapStore.zoom);
export const mapBearing = derived(mapStore, ($mapStore) => $mapStore.bearing);
