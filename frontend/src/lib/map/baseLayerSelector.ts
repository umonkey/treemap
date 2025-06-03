import { baseLayer, droneLayer, mapLayerStore } from '$lib/stores/mapLayerStore';
import { clusterStore } from '$lib/stores/clusterStore';
import L from 'leaflet';
import { get } from 'svelte/store';

const getDefaultLayer = (): string => {
	const isDark = window?.matchMedia('(prefers-color-scheme: dark)')?.matches ?? false;
	return isDark ? 'OSM Dark' : 'OSM Light';
};

export const addLayerSelection = (map: L.Map) => {
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

	const osmBasic = L.tileLayer(
		'https://tile.openstreetmap.org/{z}/{x}/{y}.png',
		{
			attribution:
				'&copy; <a href="https://myga.am/">Kanach Yerevan</a> &amp; <a href="https://www.openstreetmap.org/copyright">OSM</a>',
			minZoom: 9,
			maxZoom: 25,
			maxNativeZoom: 19
		}
	);

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

	const cluster = L.layerGroup([]).addTo(map);

	const baseMaps = {
		'OSM Dark': osmDark,
		'OSM Light': osmLight,
		'OSM Basic': osmBasic,
		Satellite: google
	};

	const overlays = {
		Drone: drone,
		Cluster: cluster
	};

	const currentLayer = get(baseLayer) ?? getDefaultLayer();

	if (currentLayer === 'OSM Dark') {
		osmDark.addTo(map);
	} else if (currentLayer === 'Satellite') {
		google.addTo(map);
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

		if (event.name === 'Cluster') {
			clusterStore.update(() => true);
		}
	});

	map.on('overlayremove', (event) => {
		if (event.name === 'Drone') {
			mapLayerStore.update((value) => {
				value.drone = false;
				return value;
			});
		}

		if (event.name === 'Cluster') {
			clusterStore.update(() => false);
		}
	});
};
