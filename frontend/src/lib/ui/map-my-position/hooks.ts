// Display current user's position on the map, if available.

import { locationStore } from '$lib/stores';
import L from 'leaflet';
import { getContext, mapKey } from '$lib/map';
import { type MountFn, type IMyPosition } from '$lib/types';
import { get, writable } from 'svelte/store';

export const hooks = (mount: MountFn) => {
	// Last displayed dot.  Removed on update.
	const dot = writable<L.Layer | null>(null);

	// Main map component, used for displaying the dot.
	const map = writable<L.Map | null>(null);

	const handleChange = (pos: IMyPosition) => {
		const m = get(map);

		if (m === null) {
			console.error('[map] MyPosition: no map context found, cannot update.');
			return;
		}

		get(dot)?.remove();

		if (pos === null) {
			dot.set(null);
			return;
		}

		console.debug(`[map] MyPosition: changed to ${pos.lat},${pos.lng}`);

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
	};

	mount(() => {
		const m = getContext<L.Map>(mapKey);

		if (m === null) {
			console.error('[map] MyPosition: no map context found, cannot start.');
			return;
		}

		map.set(m);

		// Update the dot whenever a location change is reported.
		const unsubscribe = locationStore.subscribe(handleChange);

		console.debug('[map] MyPosition: started, will update on location changes.');

		// Initialize the dot if the position is known.
		const last = get(locationStore);

		if (last !== null) {
			handleChange(last);
		}

		return () => {
			unsubscribe();
			get(dot)?.remove();
			map.set(null);
		};
	});
};
