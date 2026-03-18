<script lang="ts">
	import type { LngLatBounds } from 'maplibre-gl';
	import { CircleLayer, GeoJSON, MapLibre } from 'svelte-maplibre';
	import 'maplibre-gl/dist/maplibre-gl.css';
	import type { ILatLng } from '$lib/types';
	import { type Snippet } from 'svelte';
	import { mapState } from './MapLibre.svelte.ts';
	import AddTree from './AddTree.svelte';

	// import { MAPTILER_KEY } from '$lib/env';
	// style = `https://api.maptiler.com/maps/openstreetmap/style.json?key=${MAPTILER_KEY}`,

	const {
		center,
		style = 'https://basemaps.cartocdn.com/gl/positron-gl-style/style.json',
		children = undefined
	} = $props<{
		center: ILatLng;
		style?: string;
		children?: Snippet;
	}>();

	let bounds: LngLatBounds | undefined = $state();

	$effect(() => {
		if (center) {
			mapState.center = center;
		}
	});
</script>

<div class="map-container">
	<MapLibre
		{style}
		bind:center={mapState.center}
		bind:zoom={mapState.zoom}
		class="map"
		bind:bounds
		onmovestart={() => mapState.handleMoveStart()}
		onmoveend={() => bounds && mapState.handleMoveEnd(bounds)}
		onzoom={() => mapState.handleZoom()}
		onload={(map) => mapState.handleMoveEnd(map.getBounds())}
		standardControls
	>
		{#if children}
			{@render children()}
		{/if}

		<AddTree onConfirm={mapState.handleAddTree} />

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
