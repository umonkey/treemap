<script>
	import { onMount } from 'svelte';
	import 'leaflet/dist/leaflet.css';
	import { Markers } from '$lib/map/markers';

	const { center, onChange, onMove, className, marker, zoom } = $props();

	let map;
	let L;

	console.debug(`[map] center=${center}, zoom=${zoom}`);

	onMount(async () => {
		L = await import('leaflet');

		map = L.map('map').setView(center, zoom ?? 15);

		L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
			attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a>',
			maxZoom: 25,
			maxNativeZoom: 19
		}).addTo(map);

		map.attributionControl.setPrefix('Kanach Yerevan');

		// Highlight the current tree.
		if (marker) {
			L.marker(marker).addTo(map);
		}

		const markers = new Markers(map, L);
		markers.onChange(onChange);

		map.on('moveend', () => {
			if (onMove) {
				onMove(map.getCenter(), map.getZoom());
			}
		});
	});
</script>

<div id="map" class={className}></div>

<style>
	#map {
		height: calc(100vh - 41px);
		z-index: 2;
	}

	@media (max-width: 480px) {
		#map {
			height: calc(100vh - 91px);
		}

		#map.treeTab {
			height: calc(100vh - 185px);
		}
	}
</style>
