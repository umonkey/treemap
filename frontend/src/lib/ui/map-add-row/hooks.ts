import L from 'leaflet';
import type { Map } from 'leaflet';
import { get, writable } from 'svelte/store';
import type { ILatLng, MountFn } from '$lib/types';
import { getMap } from '$lib/map';

type ConfirmFn = (start: ILatLng, end: ILatLng) => void;

const getDistance = (p1: ILatLng, p2: ILatLng): number => {
	// Calculate the distance between two points in meters.
	const R = 6371000; // Radius of the Earth in meters
	const φ1 = (p1.lat * Math.PI) / 180; // φ, λ in radians
	const φ2 = (p2.lat * Math.PI) / 180;
	const Δφ = ((p2.lat - p1.lat) * Math.PI) / 180;
	const Δλ = ((p2.lng - p1.lng) * Math.PI) / 180;

	const a =
		Math.sin(Δφ / 2) * Math.sin(Δφ / 2) +
		Math.cos(φ1) * Math.cos(φ2) * Math.sin(Δλ / 2) * Math.sin(Δλ / 2);
	const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));

	return Math.round(R * c * 100) / 100; // Distance in meters
};

export const hooks = ({ onMount }: { onMount: MountFn }) => {
	// Starting point of the row.
	const start = writable<ILatLng | null>(null);

	// Ending point of the row.
	const end = writable<ILatLng | null>(null);

	// The line representing the row.  Needs to be removed
	// and updated on each map move.
	const line = writable<L.Polyline | null>(null);

	// The button control, remove on unmount.
	const button = writable<L.Control | null>(null);

	// Flags that the row has been started.
	const distance = writable<number | null>(null);

	// Map instance, set during initialization.
	let map: Map;

	const updateLine = (map: Map, p1: ILatLng, p2: ILatLng) => {
		get(line)?.remove();
		line.set(null);

		const l = L.polyline([p1, p2], {
			color: 'blue',
			weight: 2,
			opacity: 0.5
		});

		l.addTo(map);

		line.set(l);

		distance.set(getDistance(p1, p2));
	};

	// Handle the start/stop button click.
	const handleClick = () => {
		if (get(start) !== null) {
			start.set(null);
			end.set(null);
			distance.set(null);
			get(line)?.remove();
			return;
		}

		const center = map.getCenter();

		start.set({
			lat: center.lat,
			lng: center.lng
		});

		console.debug(`[map] Row start updated: ${center.lat},${center.lng}.`);
	};

	const handleMove = () => {
		const p1 = get(start);

		if (p1 === null) {
			console.debug('[MapAddRow] Map moved, but row not started.');
			return;
		}

		const center = map.getCenter();

		const p2 = {
			lat: center.lat,
			lng: center.lng
		};

		end.set(p2);
		updateLine(map, p1, p2);
	};

	onMount(() => {
		map = getMap();

		map.on('moveend', handleMove);

		return () => {
			get(button)?.remove();
			get(line)?.remove();

			start.set(null);
			end.set(null);

			map.off('moveend', handleMove);
		};
	});

	const handleConfirm = (confirmFn: ConfirmFn) => {
		const p1 = get(start);
		const p2 = get(end);

		if (p1 && p2) {
			confirmFn(p1, p2);
		}
	};

	return { start, distance, handleClick, handleConfirm };
};
