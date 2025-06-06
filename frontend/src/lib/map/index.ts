import type { Map } from 'leaflet';
import type { ILatLng } from '$lib/types';
export { mapHome } from './home';
export { getContext } from 'svelte';
import { getContext } from 'svelte';

// This is used for context access by Map plugins.
export const mapKey = Symbol();

export const getMap = (): Map => {
	const map = getContext<Map>(mapKey);

	if (!map) {
		throw new Error('Map context not found.');
	}

	return map;
};

export const spreadDots = (start: ILatLng, end: ILatLng, count: number): ILatLng[] => {
	const res = [];

	const latStep = (end.lat - start.lat) / (count - 1);
	const lngStep = (end.lng - start.lng) / (count - 1);

	for (let i = 0; i < count; i++) {
		res.push({
			lat: start.lat + latStep * i,
			lng: start.lng + lngStep * i
		});
	}

	return res;
};
