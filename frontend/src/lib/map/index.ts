import type { ILatLng } from '$lib/types';
export { mapHome } from './home';
export { getContext } from 'svelte';

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
