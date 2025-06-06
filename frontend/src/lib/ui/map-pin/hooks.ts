import L, { type Map } from 'leaflet';
import PIN_IMAGE from '$lib/assets/marker-icon-2x.png';
import type { ILatLng, MountFn } from '$lib/types';
import { get, writable } from 'svelte/store';
import { mapKey, getContext } from '$lib/map';

export const hooks = (mount: MountFn) => {
	const map = writable<Map | null>(null);

	const pin = writable<L.Marker | null>(null);

	mount(() => {
		const m = getContext<Map>(mapKey) ?? null;

		if (m === null) {
			console.error('[MapPin] No map context found.');
			return;
		}

		map.set(m);
	});

	const handleChange = (pos: ILatLng) => {
		get(pin)?.remove();

		const m = get(map);

		if (m === null) {
			console.error('[MapPin] Map is not initialized');
			return;
		}

		const p = L.marker([pos.lat, pos.lng], {
			icon: L.icon({
				iconUrl: PIN_IMAGE,
				iconSize: [25, 41],
				iconAnchor: [12, 41]
			})
		}).addTo(m);

		pin.set(p);
	};

	return { handleChange };
};
