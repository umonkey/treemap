<script lang="ts">
	import 'leaflet/dist/leaflet.css';
	import type { ITree } from '$lib/types';
	import { Markers } from '$lib/map/markers';
	import { addLayerSelection } from '$lib/map/baseLayerSelector';
	import { addLocateMeButton } from '$lib/map/addLocateMeButton';
	import { addLocateMeCircle } from '$lib/map/addLocateMeCircle';
	import { addResizeObserver } from '$lib/map/resizeObserver';
	import { addTreeButton } from '$lib/map/addTreeButton';
	import { baseLayer } from '$lib/stores/mapLayerStore';
	import { locationBus } from '$lib/buses/locationBus';
	import { onDestroy, onMount } from 'svelte';
	import { hook } from './hooks';

	const {
		center,
		onMove,
		className = 'default',
		marker,
		zoom = 15,
		searchQuery = undefined,
		crosshair = false,
		canAdd = false
	} = $props<{
		center: [number, number];
		onMove: (center: [number, number], zoom: number) => void;
		className: string;
		marker?: [number, number];
		zoom: number;
		searchQuery?: string | undefined;
		crosshair?: boolean | undefined;
		canAdd?: boolean | undefined;
	}>();

	const { map, mount, destroy, update, handleMarkers } = hook('map');

	onMount(async () => {
		console.debug('[Map.svelte] MOUNT');

		mount({ center, zoom });

		addLayerSelection($map);
		addResizeObserver($map);
		addLocateMeCircle($map);
		addLocateMeButton($map);

		if (canAdd) {
			addTreeButton($map);
		}

		$map.attributionControl.setPrefix('');

		const markers = new Markers($map, searchQuery);

		$map.on('moveend', () => {
			if (onMove) {
				const ll = $map.getCenter();
				onMove([ll.lat, ll.lng], $map.getZoom());
			}
		});

		// Start tracking user's location.
		locationBus.emit('start');
	});

	onDestroy(destroy);

	$effect(() =>
		update({
			center,
			zoom,
			marker
		})
	);

	$effect(() => {
		handleMarkers(marker);
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
