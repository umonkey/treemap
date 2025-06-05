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
import type { ILatLng, MountFn, DestroyFn } from '$lib/types';
import { MAX_BOUNDS } from '$lib/constants';
import { Markers } from '$lib/map/markers';
import { addLocateMeButton } from '$lib/map/addLocateMeButton';
import { addResizeObserver } from '$lib/map/resizeObserver';
import { get } from 'svelte/store';
import { locationBus, mapBus } from '$lib/buses';
import { mapCenter, mapZoom, mapStore } from '$lib/stores/mapStore';
import { writable } from 'svelte/store';
import { mapKey } from '$lib/map';
import { setContext } from 'svelte';

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

export const hook = (element: string, mount: MountFn, destroy: DestroyFn) => {
	const map = writable<Map | null>(null);
	const lastMarkerPos = writable<[number, number] | null>(null);
	const lastMarkerElement = writable<Marker | null>(null);
	const markers = writable<Markers | undefined>(undefined);

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

		addResizeObserver(em);
		addLocateMeButton(em);

		markers.set(new Markers(em, undefined));

		// Set up bus handlers.
		mapBus.on('center', handleCenter);

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

			mapStore.update((state) => {
				return {
					...state,
					center: {
						lat: c.lat,
						lng: c.lng
					},
					zoom: em.getZoom()
				};
			});
		});

		// Set the map context for plugins.
		setContext(mapKey, em);
		console.debug('[map] Map context set.');
	});

	// Clean up on component removal.
	destroy(() => {
		console.debug('[map] Destroying the map component.');
		mapBus.off('center', handleCenter);
		get(map)?.remove();
	});

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

	const handleSearch = (value: string | undefined) => {
		get(markers)?.setSearchQuery(value);
	};

	const handleElementChange = (element: HTMLDivElement | undefined) => {
		console.debug('[Map] Changing map element', element);
	};

	return {
		map,
		destroy,
		update,
		handleCenter,
		handleSearch,
		handleElementChange
	};
};
