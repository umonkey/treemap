<script lang="ts">
	import { MAX_BOUNDS } from '$lib/constants';
	import type { ILatLng } from '$lib/types';
	import { AttributionControl, MapLibre, NavigationControl } from 'svelte-maplibre';
	import { RasterLayer, RasterTileSource } from 'svelte-maplibre';
	import 'maplibre-gl/dist/maplibre-gl.css';
	import { type Snippet, onMount } from 'svelte';
	import BarsButton from './BarsButton.svelte';
	import LayerButton from './LayerButton.svelte';
	import LocateButton from './LocateButton.svelte';
	import LocationTracker from './LocationTracker.svelte';
	import { mapState } from './MapLibre.svelte.ts';
	import Marker from './Marker.svelte';
	import SearchControl from './SearchControl.svelte';
	import TreeLayer from './TreeLayer.svelte';

	const { children = undefined, onMove } = $props<{
		children?: Snippet;
		onMove?: (ll: ILatLng) => void;
	}>();

	onMount(mapState.onMount);

	$effect(() => {
		mapState.onMove = onMove;
	});
</script>

<div class="map-container">
	<MapLibre
		style={mapState.layer}
		bind:map={mapState.map}
		bind:center={mapState.center}
		bind:zoom={mapState.zoom}
		class="map"
		bind:bounds={mapState.bounds}
		onmoveend={mapState.handleMoveEnd}
		onzoom={mapState.handleZoom}
		onload={mapState.handleLoad}
		maxBounds={MAX_BOUNDS}
		attributionControl={false}
	>
		<NavigationControl position="top-left" />
		<LocateButton />
		<BarsButton />
		<LocationTracker />
		<AttributionControl compact={true} position="bottom-left" />

		{#if children}
			{@render children()}
		{/if}

		<LayerButton />

		{#if mapState.marker}
			<Marker center={mapState.marker} />
		{/if}

		{#if mapState.droneLayer}
			<RasterTileSource id="drone-source" tiles={[mapState.droneLayer]} tileSize={128} scheme="tms">
				<RasterLayer
					id="drone-layer"
					paint={{
						'raster-opacity': 0.75
					}}
				/>
			</RasterTileSource>
		{/if}

		<TreeLayer />
	</MapLibre>

	<SearchControl />
</div>

<style>
	.map-container {
		position: relative;
		width: 100%;
		height: 100%;
		z-index: var(--z-map);
	}

	:global(.map) {
		width: 100%;
		height: 100%;
	}

	:global(.maplibregl-ctrl-top-left) {
		top: env(safe-area-inset-top);
		left: env(safe-area-inset-left);
	}
	:global(.maplibregl-ctrl-top-right) {
		top: env(safe-area-inset-top);
		right: env(safe-area-inset-right);
	}
	:global(.maplibregl-ctrl-bottom-left) {
		bottom: env(safe-area-inset-bottom);
		left: env(safe-area-inset-left);
	}
	:global(.maplibregl-ctrl-bottom-right) {
		bottom: env(safe-area-inset-bottom);
		right: env(safe-area-inset-right);
	}

	@media screen and (max-width: 600px) {
		.map-container {
			height: 100%;
		}
	}
</style>
