// Display current user's position on the map, if available.

import { locationStore } from '$lib/stores/locationStore';
import L from 'leaflet';
import { getContext, mapKey } from '$lib/map';
import { type MountFn } from '$lib/types';
import { get, writable } from 'svelte/store';

export const hooks = (mount: MountFn) => {
	const dot = writable<L.Layer | null>(null);

	const map = writable<L.Map | null>(null);

	// Update the dot whenever a location change is reported.
	const unsubscribe = locationStore.subscribe((pos) => {
		const m = get(map);

		if (m === null) {
			console.error('[MapMyPosition] No map context found.');
			return;
		}

		get(dot)?.remove();

		if (pos === null) {
			dot.set(null);
			return;
		}

		console.debug(`[map] Location changed to ${pos.lat},${pos.lng}`);

		dot.set(
			L.circleMarker(pos, {
				weight: 2,
				color: '#fff',
				fillColor: '#2A93EE',
				fillOpacity: 1,
				opacity: 1,
				radius: 9
			}).addTo(m)
		);
	});

	mount(() => {
		const m = getContext<L.Map>(mapKey);

		if (m === null) {
			console.error('[MapMyPosition] No map context found.');
			return;
		}

		map.set(m);

		return () => {
			unsubscribe();
		};
	});
};
