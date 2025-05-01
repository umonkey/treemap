// Map component logic.
// Please note that the map is very complex and prop based control doesn't really work well for this.
// When you need to present a pre-configured map, this is OK, but when you need to move the map
// programmatically, add and remove layers on the go, props just aren't very handy for that.
//
// This is especially bad when the component has a lot of props, and the change effect fires
// on every change of any prop, so we need to manually keep track of which ones are really changing.
// Which is doable, but rather complex.
//
// So we use the map bus instead.

import L from 'leaflet';
import type { Map, Marker } from 'leaflet';
import { MAX_BOUNDS } from '$lib/constants';
import { get } from 'svelte/store';
import { writable } from 'svelte/store';
import { mapBus } from '$lib/buses';

const getMaxBounds = () => {
	const c1 = L.latLng(MAX_BOUNDS[0][0], MAX_BOUNDS[0][1]);
	const c2 = L.latLng(MAX_BOUNDS[1][0], MAX_BOUNDS[1][1]);

	return L.latLngBounds(c1, c2);
};

const lldiff = (a: [number, number] | null, b: [number, number] | null): boolean => {
	const _a = a || [0, 0];
	const _b = b || [0, 0];
	return _a[0] !== _b[0] || _a[1] !== _b[1];
};

export const hook = (element: string) => {
	const map = writable<Map | null>(null);
	const lastMarkerPos = writable<[number, number] | null>(null);
	const lastMarkerElement = writable<Marker | null>(null);

	// Initialize the map component.
	const mount = ({ center, zoom }: { center: [number, number]; zoom: number }) => {
		const em = L.map(element, {
			maxBounds: getMaxBounds()
		}).setView(center, zoom);

		map.set(em);

		// Set up bus handlers.
		mapBus.on('center', handleCenter);
	};

	// Destroy the map component.
	const destroy = () => {
		console.debug('[map] Destroying map');
		get(map)?.remove();

		// TODO: remove bus handlers.
	};

	// Update the component when parameters change.
	const update = ({
		center,
		zoom,
		marker
	}: {
		center: [number, number];
		zoom: number;
		marker: [number, number] | null;
	}) => {
		const m = get(map);

		if (m === null) {
			console.error('[map] Map is not initialized');
			return; // never happens
		}

		if (center[0] != m.getCenter().lat || center[1] != m.getCenter().lng || zoom != m.getZoom()) {
			console.debug('[map] Updating map center', center, m.getCenter());
			m.panTo(center);
			// m.setView(center, zoom);
		}

		if (lldiff(get(lastMarkerPos), marker)) {
			console.debug(`[map] Updating marker to ${marker}`);

			const removeMarker = get(lastMarkerElement);

			if (marker) {
				const ctl = L.marker(marker, {
					icon: L.icon({
						iconUrl: '/icons/marker-icon-2x.png',
						iconSize: [25, 41],
						iconAnchor: [12, 41]
					})
				}).addTo(m);

				lastMarkerPos.set(marker);
				lastMarkerElement.set(ctl);
			} else {
				lastMarkerPos.set(null);
				lastMarkerElement.set(null);
			}

			if (removeMarker) {
				m.removeLayer(removeMarker);
			}
		}
	};

	const handleCenter = (pos: [number, number]) => {
		console.debug(`[map] Request to center: ${pos}`);
		get(map)?.panTo(pos);
	};

	const handleMarkers = (marker?: [number, number]) => {
		console.debug(`[map] handleMarkers: ${marker}`);
	};

	return { map, mount, destroy, update, handleCenter, handleMarkers };
};
