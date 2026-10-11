import { DEFAULT_BBOX } from '$lib/constants';
import { showError } from '$lib/errors';
import { locale } from '$lib/locale';
import type { ILatLng } from '$lib/types';
import { bboxCenter } from '$lib/utils/geo';
import { ls } from '$lib/utils/localStorage';
import { derived, writable } from 'svelte/store';

const STORE_ID = 'mapStore-v4';

type IMapStore = {
	center: ILatLng;
	zoom: number;
	bearing: number;
};

const stored = ls.read<IMapStore>(STORE_ID);

export const hasStoredMapView = stored !== undefined;

export const mapStore = writable<IMapStore>(
	stored || {
		center: bboxCenter(DEFAULT_BBOX),
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
