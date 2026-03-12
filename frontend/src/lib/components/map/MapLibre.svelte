<script lang="ts">
	import type { LngLatBounds } from 'maplibre-gl';
	import { FillLayer, GeoJSON, MapLibre } from 'svelte-maplibre';
	import 'maplibre-gl/dist/maplibre-gl.css';
	import type { ILatLng } from '$lib/types';
	import { type Snippet, onMount } from 'svelte';
	import { mapState } from './MapLibre.svelte.ts';

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

	onMount(mapState.handleMount);
</script>

<div class="map-container">
	<MapLibre
		{style}
		{center}
		bind:zoom={mapState.zoom}
		class="map"
		bind:bounds
		onmovestart={() => mapState.handleMoveStart()}
		onmoveend={() => bounds && mapState.handleMoveEnd(bounds)}
		onzoom={() => mapState.handleZoom()}
		standardControls
	>
		{#if children}
			{@render children()}
		{/if}

		{#if mapState.markers}
			<GeoJSON data={mapState.markers}>
				<FillLayer
					id="tree-canopies"
					filter={['==', ['get', 'type'], 'canopy']}
					paint={{
						'fill-color': [
							'match',
							['get', 'state'],
							'stump',
							'#000000',
							'gone',
							'#000000',
							'unknown',
							'#ffd700',
							'dead',
							'#8b4513',
							'#228b22' // default
						],
						'fill-opacity': 0.5
					}}
					onclick={mapState.handleClick}
				/>

				<FillLayer
					id="tree-trunks"
					filter={['==', ['get', 'type'], 'trunk']}
					paint={{
						'fill-color': '#000000',
						'fill-opacity': 0.5
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
