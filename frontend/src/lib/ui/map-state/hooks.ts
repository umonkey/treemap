// Track map movement, save current state in local storage.
// The state is then used to initialize the default map.

import { type Map } from 'leaflet';
import { getMap } from '$lib/map';
import { mapStore } from '$lib/stores/mapStore';
import { type MountFn } from '$lib/types';

export const hooks = ({ onMount }: { onMount: MountFn }) => {
	let map: Map;

	const handleMove = () => {
		console.debug('[map] Map moved, saving state.');

		const center = map.getCenter();
		const zoom = map.getZoom();

		mapStore.update((state) => ({
			...state,
			center: {
				lat: center.lat,
				lng: center.lng
			},
			zoom
		}));
	};

	onMount(() => {
		map = getMap();
		map.on('moveend', handleMove);

		return () => {
			map.off('moveend', handleMove);
		};
	});
};
