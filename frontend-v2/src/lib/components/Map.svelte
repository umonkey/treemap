<script>
	import { onMount } from 'svelte';
	import 'leaflet/dist/leaflet.css';
	import { onMapMoveEnd } from '$lib/handlers/mapEvents';

	export let center = [51.505, -0.09];

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

		map.on('moveend', () => onMapMoveEnd(map));
	});
</script>

<div id="map"></div>

<style>
	#map {
		height: 400px;
	}
</style>
