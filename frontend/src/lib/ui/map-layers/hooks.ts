import L, { type Map } from 'leaflet';
import type { MountFn } from '$lib/types';
import { MAPTILER_KEY } from '$lib/env';
import { MaptilerLayer } from '@maptiler/leaflet-maptilersdk';
import { baseLayer, droneLayer, mapLayerStore } from '$lib/stores/mapLayerStore';
import { get } from 'svelte/store';
import { mapKey, getContext } from '$lib/map';

const getDefaultLayer = (): string => {
	const isDark = window?.matchMedia('(prefers-color-scheme: dark)')?.matches ?? false;
	return isDark ? 'OSM Dark' : 'OSM Light';
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

		const osmBasic = new MaptilerLayer({
			apiKey: MAPTILER_KEY,
			style: 'openstreetmap',
			language: ' en'
		});

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
