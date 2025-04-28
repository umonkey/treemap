import L from 'leaflet';
import { writable } from 'svelte/store';
import { MAX_BOUNDS } from '$lib/constants';

const getMaxBounds = () => {
	const c1 = L.latLng(MAX_BOUNDS[0][0], MAX_BOUNDS[0][1]);
	const c2 = L.latLng(MAX_BOUNDS[1][0], MAX_BOUNDS[1][1]);

	return L.latLngBounds(c1, c2);
};

export const hook = (element: string) => {
	const map = writable<L.Map | null>(null);

	map.set(
		L.map(element, {
			maxBounds: getMaxBounds()
		})
	);
};
