// This helper displays pins on the map.

import L from 'leaflet';
import type { Map, Marker } from 'leaflet';
import type { ILatLng } from '$lib/types';
import { writable } from 'svelte/store';
import PIN_IMAGE from '$lib/assets/marker-icon-2x.png';
import { get } from 'svelte/store';

export const addPins = () => {
	const existing = writable<Marker[]>([]);

	const updatePins = (map: Map, pins: ILatLng[]) => {
		const newMarkers = pins.map((pin: ILatLng) => {
			const ctl = L.marker([pin.lat, pin.lng], {
				icon: L.icon({
					iconUrl: PIN_IMAGE,
					iconSize: [25, 41],
					iconAnchor: [12, 41]
				})
			}).addTo(map);

			return ctl;
		});

		get(existing).forEach((marker) => {
			map.removeLayer(marker);
		});

		existing.set(newMarkers);
	};

	return { updatePins };
};
