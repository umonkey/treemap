<script lang="ts">
	import { MAX_BOUNDS } from '$lib/constants';
	import type { ILatLng } from '$lib/types';
	import { AttributionControl, MapLibre } from 'svelte-maplibre';
	import { RasterLayer, RasterTileSource } from 'svelte-maplibre';
	import 'maplibre-gl/dist/maplibre-gl.css';
	import { type Snippet, onMount } from 'svelte';
	import LayerButton from './LayerButton.svelte';
	import LocateButton from './LocateButton.svelte';
	import LocationTracker from './LocationTracker.svelte';
	import { mapState } from './MapLibre.svelte.ts';
	import Marker from './Marker.svelte';
	import Navigation from './Navigation.svelte';
	import SearchControl from './SearchControl.svelte';
	import TreeLayer from './TreeLayer.svelte';
	import AlertLayer from './AlertLayer.svelte';
	import PanoramicLayer from './PanoramicLayer.svelte';
	import TreeHintsLayer from './TreeHintsLayer.svelte';
	import WaterSourceLayer from './WaterSourceLayer.svelte';
	import AddButton from './AddButton.svelte';
	import MapCenter from './MapCenter.svelte';
	import MapMenuBridge from './MapMenuBridge.svelte';
	import MapEventBridge from './MapEventBridge.svelte';
	import MapRays from './MapRays.svelte';
	import MoveLine from './MoveLine.svelte';
	import NearestTree from './NearestTree.svelte';
	import NearestPoi from './NearestPoi.svelte';
	import MapRowPreview from './MapRowPreview.svelte';
	import { mapMode } from '$lib/stores/mapMode';
	import { centerLayer } from '$lib/stores/mapLayerStore';

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
		bind:bearing={mapState.bearing}
		pitch={0}
		maxPitch={0}
		class="map"
		bind:bounds={mapState.bounds}
		onmoveend={mapState.handleMoveEnd}
		onzoom={mapState.handleZoom}
		onrotate={mapState.handleRotate}
		onload={mapState.handleLoad}
		maxBounds={MAX_BOUNDS}
		attributionControl={false}
	>
		<LocateButton />
		<LocationTracker />
		<MapMenuBridge />
		<MapEventBridge />
		<AttributionControl compact={true} position="bottom-left" />

		{#if children}
			{@render children()}
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

		<LayerButton />

		<Marker />

		<AddButton />

		<MapRays />

		<TreeLayer />

		<AlertLayer />

		<PanoramicLayer />

		<TreeHintsLayer />

		<WaterSourceLayer />

		{#if $mapMode === 'move' || $mapMode === 'add' || $mapMode === 'add-row'}
			<MapCenter />
			{#if $mapMode === 'move'}
				<MoveLine />
			{:else if $mapMode === 'add'}
				<NearestTree />
			{:else if $mapMode === 'add-row'}
				<MapRowPreview />
				<NearestTree />
			{/if}
		{:else if $mapMode === undefined || $mapMode === 'preview'}
			{#if $centerLayer}
				<MapCenter />
			{/if}
			<NearestPoi zoom={mapState.zoom} distance={5} />
		{/if}
	</MapLibre>

	<Navigation />
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

	.map-container :global(.maplibregl-canvas-container.maplibregl-interactive) {
		cursor: crosshair;
	}

	.map-container :global(.maplibregl-canvas-container.maplibregl-interactive:active) {
		cursor: grabbing;
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
		bottom: 0;
		left: env(safe-area-inset-left);
	}
	:global(.maplibregl-ctrl-bottom-right) {
		bottom: 0;
		right: env(safe-area-inset-right);
	}

	@media screen and (max-width: 1023px) {
		.map-container {
			height: calc(100% - var(--bottom-nav-height));
		}
	}
</style>
