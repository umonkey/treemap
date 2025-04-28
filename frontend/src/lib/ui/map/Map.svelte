<script lang="ts">
	import 'leaflet/dist/leaflet.css';
	import type { ITree } from '$lib/types';
	import type { Map } from 'leaflet';
	import { MAX_BOUNDS } from '$lib/constants';
	import { Markers } from '$lib/map/markers';
	import { addLayerSelection } from '$lib/map/baseLayerSelector';
	import { addLocateMeButton } from '$lib/map/addLocateMeButton';
	import { addLocateMeCircle } from '$lib/map/addLocateMeCircle';
	import { addResizeObserver } from '$lib/map/resizeObserver';
	import { addTreeButton } from '$lib/map/addTreeButton';
	import { baseLayer } from '$lib/stores/mapLayerStore';
	import { locationBus } from '$lib/buses/locationBus';
	import { onDestroy, onMount } from 'svelte';

	const {
		center,
		onChange,
		onMove,
		className = 'default',
		marker,
		zoom = 15,
		searchQuery = undefined,
		crosshair = false,
		canAdd = false
	} = $props<{
		center: [number, number];
		onChange: (tree: ITree) => void;
		onMove: (center: [number, number], zoom: number) => void;
		className: string;
		marker?: [number, number] | undefined;
		zoom: number;
		searchQuery?: string | undefined;
		crosshair?: boolean | undefined;
		canAdd?: boolean | undefined;
	}>();

	$effect(() => {
		console.debug(`[map] Map.svelte effect`, marker);
	});

	let map: Map;

	// biome-ignore lint/suspicious/noImplicitAnyLet: Leaflet :(
	let L;

	onMount(async () => {
		L = await import('leaflet');

		const c1 = L.latLng(MAX_BOUNDS[0][0], MAX_BOUNDS[0][1]);
		const c2 = L.latLng(MAX_BOUNDS[1][0], MAX_BOUNDS[1][1]);

		map = L.map('map', {
			maxBounds: L.latLngBounds(c1, c2)
		}).setView(center, zoom);

		addLayerSelection(map);
		addResizeObserver(map);
		addLocateMeCircle(map);
		addLocateMeButton(map);

		if (canAdd) {
			addTreeButton(map);
		}

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

		// Start tracking user's location.
		locationBus.emit('start');
	});

	onDestroy(() => {
		console.debug('[map] Destroying map.');
		map.remove();
	});
</script>

<div
	id="map"
	class={className}
	class:dark={$baseLayer === 'OSM Dark'}
	class:light={$baseLayer !== 'OSM Dark'}
></div>

{#if crosshair}
	<img src="/icons/crosshair.svg" class="crosshair" alt="" />
{/if}

<style>
	#map {
		height: 100%;
		width: 100%;
		z-index: 1;
	}

	#map.light {
		background-color: #f9f6ef;
	}

	#map.dark {
		background-color: #080808;
	}

	.crosshair {
		width: 30px;
		height: 30px;
		position: absolute;
		left: 50%;
		top: 50%;
		z-index: 50;
		transform: translate(-50%, -50%);
	}

	:global {
		.cluster-count {
			background-color: transparent;
			border: none;
			box-shadow: none;
			font-size: 14px;
			line-height: 40px;
			text-align: center;
			padding: 0;
			color: #040;
		}
	}

	@media (max-width: 480px) {
		:global {
			.cluster-count {
				font-size: 14px;
			}
		}
	}

	@media (prefers-color-scheme: dark) {
		:global {
			.cluster-count {
				color: #000;
			}

			#map.dark .cluster-count {
				color: #fff;
			}
		}
	}
</style>
