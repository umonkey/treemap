<script>
	import { onMount } from 'svelte';
	import 'leaflet/dist/leaflet.css';
	import { Markers } from '$lib/map/markers';

	const { center, onChange, className } = $props();

	let map;
	let L;

	onMount(async () => {
		L = await import('leaflet');

		map = L.map('map').setView(center, 18);

		L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
			attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a>',
			maxZoom: 25,
			maxNativeZoom: 19
		}).addTo(map);

		map.attributionControl.setPrefix('Kanach Yerevan');

		const markers = new Markers(map, L);
		markers.onChange(onChange);
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
