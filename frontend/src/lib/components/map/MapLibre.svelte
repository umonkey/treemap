<script lang="ts">
	import { MAX_BOUNDS } from '$lib/constants';
	import type { ILatLng } from '$lib/types';
	import type { LngLatBounds } from 'maplibre-gl';
	import { CircleLayer, GeoJSON, MapLibre } from 'svelte-maplibre';
	import { RasterLayer, RasterTileSource } from 'svelte-maplibre';
	import 'maplibre-gl/dist/maplibre-gl.css';
	import { type Snippet, onMount } from 'svelte';
	import { mapState } from './MapLibre.svelte.ts';
	import Marker from './Marker.svelte';

	const { children = undefined, onMove } = $props<{
		children?: Snippet;
		onMove?: (ll: ILatLng) => void;
	}>();

	let bounds: LngLatBounds | undefined = $state();

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
		bind:bounds
		onmovestart={mapState.handleMoveStart}
		onmoveend={() => bounds && mapState.handleMoveEnd(bounds)}
		onzoom={mapState.handleZoom}
		onload={mapState.handleLoad}
		maxBounds={MAX_BOUNDS}
		standardControls
	>
		{#if children}
			{@render children()}
		{/if}

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

		{#if mapState.markers}
			<GeoJSON data={mapState.markers}>
				<CircleLayer
					id="tree-crowns"
					paint={{
						'circle-color': [
							'match',
							['get', 'state'],
							['stump', 'gone'],
							'#000000',
							'unknown',
							'#ffd700',
							'dead',
							'#8b4513',
							'#228B22'
						],
						'circle-radius': [
							'interpolate',
							['exponential', 2],
							['zoom'],
							10,
							['*', ['get', 'crown'], 0.00428],
							22,
							['*', ['get', 'crown'], 17.534]
						],
						'circle-opacity': ['match', ['get', 'state'], ['stump', 'gone'], 0.2, 0.5],
						'circle-pitch-alignment': 'map',
						'circle-pitch-scale': 'map'
					}}
					onclick={mapState.handleClick}
				/>

				<CircleLayer
					id="tree-trunks"
					paint={{
						'circle-color': '#000000',
						'circle-radius': [
							'interpolate',
							['exponential', 2],
							['zoom'],
							10,
							['*', ['get', 'trunk'], 0.00428],
							22,
							['*', ['get', 'trunk'], 17.534]
						],
						'circle-opacity': 0.8,
						'circle-pitch-alignment': 'map',
						'circle-pitch-scale': 'map'
					}}
				/>
			</GeoJSON>
		{/if}
	</MapLibre>
</div>

<style>
	.map-container {
		width: 100%;
		height: 100%;
		position: relative;
	}

	:global(.map) {
		width: 100%;
		height: 100%;
	}
</style>
