<script lang="ts">
	import 'leaflet/dist/leaflet.css';
	import { baseLayer } from '$lib/stores/mapLayerStore';
	import { onDestroy, onMount, type Snippet } from 'svelte';
	import { hook } from './hooks';
	import {
		MapLayers,
		MapMyPosition,
		MapResizeObserver,
		MapState,
		MapMarkerLoader,
		MapTrees
	} from '$lib/ui';

	const {
		center,
		className = 'default',
		children = undefined
	} = $props<{
		center: [number, number];
		className: string;
		children?: Snippet | undefined;
	}>();

	const { handleCenter } = hook('map', onMount, onDestroy);

	// We need this to track when the map is ready, so we can render children.
	let map: HTMLDivElement = $state<HTMLDivElement | undefined>(undefined);

	$effect(() => handleCenter(center));
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
		<MapMyPosition />
		<MapState />
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
