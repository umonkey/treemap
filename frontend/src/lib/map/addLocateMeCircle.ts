/**
 * Track my location and add to the current map.
 */

import { locationStore } from '$lib/stores/locationStore';
import L from 'leaflet';

export const addLocateMeCircle = (map: L.Map) => {
	console.debug('[map] Adding my location display.');

	let dot: L.Layer | null = null;

	const unsubscribe = locationStore.subscribe((pos) => {
		if (dot) {
			map.removeLayer(dot);
			dot = null;
		}

		if (pos === null) {
			return;
		}

		console.debug(`[map] Location changed to ${pos.lat},${pos.lng}`);

		dot = L.circleMarker(pos, {
			weight: 2,
			color: '#fff',
			fillColor: '#2A93EE',
			fillOpacity: 1,
			opacity: 1,
			radius: 9
		}).addTo(map);
	});

	map.on('unload', () => {
		console.debug('[map] Removing my location display.');
		unsubscribe();
	});
};
