<script lang="ts">
	import 'leaflet/dist/leaflet.css';
	import type { Map } from 'leaflet';
	import type { ITree } from '$lib/types';
	import { addLayerSelection } from '$lib/map/baseLayerSelector';
	import { Markers } from '$lib/map/markers';
	import { onMount } from 'svelte';

	const {
		center,
		onChange = (tree: ITree) => {
			console.debug(`[map] Selected tree ${tree.id}`);
		},
		onMove = () => {},
		className = 'default',
		marker = undefined,
		zoom,
		searchQuery = undefined
	} = $props();

	let map: Map;
	let L;

	console.debug(`[map] center=${center}, zoom=${zoom} search=${searchQuery}`);

	onMount(async () => {
		L = await import('leaflet');

		map = L.map('map').setView(center, zoom ?? 15);

		addLayerSelection(map);

		map.attributionControl.setPrefix('');

		// Highlight the current tree.
		if (marker) {
			L.marker(marker, {
				icon: L.icon({
					iconUrl: '/icons/marker-icon-2x.png',
					iconSize: [25, 41],
					iconAnchor: [12, 41]
				})
			}).addTo(map);
		}

		const markers = new Markers(map, searchQuery);

		markers.onChange((tree: ITree) => {
			onChange(tree);
		});

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
		z-index: var(--z-map);
	}

	@media (max-width: 480px) {
		#map {
			height: calc(100vh - 91px);
		}

		#map.treeTab {
			height: calc(100vh - 185px);
		}
	}

	@media (prefers-color-scheme: dark) {
		#map {
			background-color: #000;
		}
	}
</style>
