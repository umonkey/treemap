import type { Map } from 'leaflet';
import { get, writable } from 'svelte/store';
import type { ILatLng, MountFn } from '$lib/types';
import { getMap } from '$lib/map';

export const hooks = ({ onMount }: { onMount: MountFn }) => {
	let map: Map;

	// Last received position, to pan to.
	const lastPosition = writable<ILatLng | null>(null);

	// Handle the button click.
	const handleClick = () => {
		const center = get(lastPosition);

		if (center !== null) {
			console.debug(`[map] Panning to last known position: ${center.lat}, ${center.lng}`);
			map.panTo([center.lat, center.lng]);
		} else {
			console.info('[map] No last known position to pan to.');
		}
	};

	onMount(() => {
		map = getMap();
	});

	return { handleClick };
};
