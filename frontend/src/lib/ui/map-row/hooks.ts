import L, { type Map } from 'leaflet';
import type { ILatLng, MountFn } from '$lib/types';
import { get, writable } from 'svelte/store';
import { getMap } from '$lib/map';

const spreadDots = (start: ILatLng, end: ILatLng, count: number): ILatLng[] => {
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

export const hooks = ({ onMount }: { onMount: MountFn }) => {
	const start = writable<ILatLng>();
	const end = writable<ILatLng>();
	const count = writable<number>(2);

	let map: Map;

	// The previous that represents the row.
	const previous = writable<L.LayerGroup | null>(null);

	const redraw = () => {
		get(previous)?.remove();

		if (!get(start) || !get(end) || get(count) < 2) {
			console.debug(`[map] Not enough data to draw the row.`);
			return;
		}

		const layer = L.layerGroup([]);

		const l = L.polyline([get(start), get(end)], {
			color: '#3f5277',
			weight: 1,
			opacity: 0.5
		});

		spreadDots(get(start), get(end), get(count)).forEach((point) => {
			const marker = L.circle(point, {
				radius: 1,
				fillColor: '#3f5277',
				color: '#3f5277',
				weight: 1,
				opacity: 1,
				fillOpacity: 0.5
			});

			marker.addTo(layer);
		});

		l.addTo(layer);

		layer.addTo(map);
		previous.set(layer);
	};

	const handleStartChange = (value: ILatLng) => {
		console.debug(`[map] Row start changed to ${value.lat}, ${value.lng}`);
		start.set(value);
		redraw();
	};

	const handleEndChange = (value: ILatLng) => {
		console.debug(`[map] Row end changed to ${value.lat}, ${value.lng}`);
		end.set(value);
		redraw();
	};

	const handleCountChange = (value: number) => {
		console.debug(`[map] Row count changed to ${value}`);
		count.set(value);
		redraw();
	};

	onMount(() => {
		map = getMap();
	});

	return { handleStartChange, handleEndChange, handleCountChange };
};
