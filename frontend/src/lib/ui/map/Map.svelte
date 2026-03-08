<script lang="ts">
	import 'leaflet/dist/leaflet.css';
	import { baseLayer } from '$lib/stores/mapLayerStore';
	import { type ILatLng } from '$lib/types';
	import {
		MapLayers,
		MapLocateMe,
		MapMarkerLoader,
		MapMyPosition,
		MapResizeObserver,
		MapState,
		MapTrees
	} from '$lib/ui';
	import { type Snippet, onMount } from 'svelte';
	import { hook } from './hooks';

	const {
		center,
		zoom = undefined,
		className = 'default',
		children = undefined
	} = $props<{
		center: ILatLng;
		zoom?: number;
		className?: string | null;
		searchQuery?: string | null;
		children?: Snippet | undefined;
	}>();

	const { handleCenter, handleZoom } = hook('map', onMount);

	// We need this to track when the map is ready, so we can render children.
	let map: HTMLDivElement | undefined = $state(undefined);

	$effect(() => handleCenter(center));
	$effect(() => {
		if (zoom !== undefined) {
			handleZoom(zoom);
		}
	});
</script>

<div class="wrapper">
	<div
		id="map"
		bind:this={map}
		class={className}
		class:dark={$baseLayer === 'OSM Dark'}
		class:light={$baseLayer !== 'OSM Dark'}
	></div>

	{#if map}
		<MapResizeObserver />
		<MapLayers />
		<MapState />

		<MapLocateMe />
		<MapMyPosition />
		<MapMarkerLoader />
		<MapTrees />

		{#if children}
			{@render children()}
		{/if}
	{/if}
</div>

<style>
	.wrapper {
		width: 100%;
		height: 100%;
		position: relative;
	}

	#map {
		height: 100%;
		width: 100%;
		z-index: 1;
		outline: none;
	}

	#map.light {
		background-color: #f9f6ef;
	}

	#map.dark {
		background-color: #080808;
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
