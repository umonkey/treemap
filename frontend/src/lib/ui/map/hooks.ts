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

import L, { type Map } from 'leaflet';
import type { ILatLng, MountFn } from '$lib/types';
import { MAX_BOUNDS } from '$lib/constants';
import { get, writable } from 'svelte/store';
import { locationBus, mapBus } from '$lib/buses';
import { mapCenter, mapZoom } from '$lib/stores/mapStore';
import { mapKey } from '$lib/map';
import { setContext } from 'svelte';

const getMaxBounds = () => {
	const c1 = L.latLng(MAX_BOUNDS[0][0], MAX_BOUNDS[0][1]);
	const c2 = L.latLng(MAX_BOUNDS[1][0], MAX_BOUNDS[1][1]);

	return L.latLngBounds(c1, c2);
};

export const hook = (element: string, mount: MountFn) => {
	const map = writable<Map | null>(null);

	// Initialize the map component on mount.
	mount(() => {
		console.debug('[map] Mounting the map component.');

		const center = get(mapCenter);
		const zoom = get(mapZoom);

		const em = L.map(element, {
			maxBounds: getMaxBounds(),
			renderer: L.canvas()
		}).setView([center.lat, center.lng], zoom);

		map.set(em);
		em.attributionControl.setPrefix('');

		// Set up bus handlers.
		mapBus.on('center', handleCenter);
		mapBus.on('fit', handleFit);

		// Start geo-location.
		//
		// Note that we don't stop it.  Once you open a map, we keep tracking your
		// location in background, so when you jump between map and other pages,
		// your location is still up to date.
		locationBus.emit('start');

		// Track and report map moves.
		em.on('moveend', () => {
			const c = em.getCenter();

			console.debug(`[map] Reporting map move to ${c.lat},${c.lng} z=${em.getZoom()}`);

			mapBus.emit('onMoved', {
				lat: c.lat,
				lon: c.lng,
				zoom: em.getZoom()
			});
		});

		// Set the map context for plugins.
		setContext(mapKey, em);
		console.debug('[map] Map context set.');

		// Disconnect handlers on shutdown.
		return () => {
			console.debug('[map] Destroying the map component.');

			mapBus.off('center', handleCenter);
			mapBus.off('fit', handleFit);

			try {
				get(map)?.remove();
			} catch (e) {
				console.error('[map] Error removing map:', e);
			}
		};
	});

	const handleCenter = (pos: ILatLng) => {
		const center = get(map)?.getCenter();

		if (!center) {
			console.error('[map] Cannot center: map not initialized.');
			return;
		}

		if (center.lat === pos.lat && center.lng === pos.lng) {
			console.debug('[map] Cannot center: already there.');
			return;
		}

		console.debug(`[map] Request to center: ${pos.lat},${pos.lng}`);
		get(map)?.panTo(pos);
	};

	// External party asking to show a particular area.
	const handleFit = (bounds: { start: ILatLng; end: ILatLng }) => {
		console.debug('[map] Request to fit bounds', bounds);
		get(map)?.fitBounds([
			[bounds.start.lat, bounds.start.lng],
			[bounds.end.lat, bounds.end.lng]
		]);
	};

	return {
		map,
		handleCenter
	};
};
