// Makes different base layers available for the map.

import L, { type Map } from 'leaflet';
import type { MountFn } from '$lib/types';
import { MAPTILER_KEY } from '$lib/env';
import { MaptilerLayer } from '@maptiler/leaflet-maptilersdk';
import { baseLayer, droneLayer, mapLayerStore } from '$lib/stores/mapLayerStore';
import { get } from 'svelte/store';
import { mapKey, getContext } from '$lib/map';

// Enables vector tiles from MapTiler.
//
// Please note that we did try this, and it looks great, zooms smoothly, but the initial rendering takes
// time.  Like when you navigate from a map page to a non-map page (e.g. home, or the new tree form),
// then back to the map, it takes seconds to render the map again.  Which is noticeable and degrades
// UX significantly.
//
// The right solution is to switch to MapLibre GL completely, which would take significant development effort,
// which we postpone for now.  For now, we just stick to raster tiles.
const MAPTILER_USE_VECTOR = false;

const getDefaultLayer = (): string => {
	const isDark = window?.matchMedia('(prefers-color-scheme: dark)')?.matches ?? false;
	return isDark ? 'OSM Dark' : 'OSM Light';
};

const getMapTilerLayer = () => {
	if (MAPTILER_USE_VECTOR) {
		return new MaptilerLayer({
			apiKey: MAPTILER_KEY,
			style: 'openstreetmap',
			language: 'en'
		});
	}

	return L.tileLayer(
		`https://api.maptiler.com/maps/openstreetmap/{z}/{x}/{y}@2x.jpg?key=${MAPTILER_KEY}`,
		{
			attribution:
				'&copy; <a href="https://myga.am/">Kanach Yerevan</a> &amp; <a href="https://www.openstreetmap.org/copyright">OSM</a>',
			minZoom: 9,
			maxZoom: 25,
			maxNativeZoom: 22
		}
	);
};

export const hooks = (mount: MountFn) => {
	const initialize = (map: Map) => {
		const osmDark = L.tileLayer(
			'https://cartodb-basemaps-{s}.global.ssl.fastly.net/dark_all/{z}/{x}/{y}@2x.png',
			{
				attribution:
					'&copy; <a href="https://myga.am/">Kanach Yerevan</a> &amp; <a href="https://www.openstreetmap.org/copyright">OSM</a>',
				minZoom: 9,
				maxZoom: 25,
				maxNativeZoom: 20
			}
		);

		const osmLight = L.tileLayer(
			'https://cartodb-basemaps-{s}.global.ssl.fastly.net/rastertiles/voyager/{z}/{x}/{y}@2x.png',
			{
				attribution:
					'&copy; <a href="https://myga.am/">Kanach Yerevan</a> &amp; <a href="https://www.openstreetmap.org/copyright">OSM</a>',
				minZoom: 9,
				maxZoom: 25,
				maxNativeZoom: 20
			}
		);

		const osmBasic = getMapTilerLayer();

		const google = L.tileLayer('http://{s}.google.com/vt/lyrs=s&x={x}&y={y}&z={z}', {
			attribution:
				'&copy; <a href="https://myga.am/">Kanach Yerevan</a> &amp; <a href="https://maps.google.com/copyright">Google</a>',
			subdomains: ['mt0', 'mt1', 'mt2', 'mt3'],
			minZoom: 9,
			maxZoom: 25,
			maxNativeZoom: 22
		});

		const drone = L.tileLayer('https://treemap-tiles.fra1.digitaloceanspaces.com/{z}/{x}/{y}.png', {
			attribution: '&copy; <a href="https://myga.am/">Kanach Yerevan</a>',
			maxZoom: 25,
			maxNativeZoom: 21,
			minZoom: 15,
			tms: true,
			opacity: 0.9
		});

		const baseMaps = {
			'OSM Dark': osmDark,
			'OSM Light': osmLight,
			'OSM Basic': osmBasic,
			Satellite: google
		};

		const overlays = {
			Drone: drone
		};

		const currentLayer = get(baseLayer) ?? getDefaultLayer();

		if (currentLayer === 'OSM Dark') {
			osmDark.addTo(map);
		} else if (currentLayer === 'Satellite') {
			google.addTo(map);
		} else if (currentLayer === 'OSM Basic') {
			osmBasic.addTo(map);
		} else {
			osmLight.addTo(map);
		}

		if (get(droneLayer)) {
			drone.addTo(map);
		}

		L.control.layers(baseMaps, overlays).addTo(map);

		map.on('baselayerchange', (event) => {
			mapLayerStore.update((value) => {
				value.base = event.name;
				return value;
			});
		});

		map.on('overlayadd', (event) => {
			if (event.name === 'Drone') {
				mapLayerStore.update((value) => {
					value.drone = true;
					return value;
				});
			}
		});

		map.on('overlayremove', (event) => {
			if (event.name === 'Drone') {
				mapLayerStore.update((value) => {
					value.drone = false;
					return value;
				});
			}
		});
	};

	mount(() => {
		const map = getContext<Map>(mapKey) ?? null;

		if (map === null) {
			console.error('[MapLayers] Map context is not available.');
			return;
		}

		initialize(map);
	});
};
