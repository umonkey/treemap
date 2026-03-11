<script lang="ts">
	import { MapLibre, CircleLayer, GeoJSON } from 'svelte-maplibre';
	import type { LngLatBounds } from 'maplibre-gl';
	import 'maplibre-gl/dist/maplibre-gl.css';
	import { MAPTILER_KEY } from '$lib/env';
	import type { ILatLng } from '$lib/types';
	import type { Snippet } from 'svelte';
	import { markers, handleMoveEnd } from './MapLibre';

	const {
		center,
		zoom = 13,
		style = `https://api.maptiler.com/maps/openstreetmap/style.json?key=${MAPTILER_KEY}`,
		children = undefined
	} = $props<{
		center: ILatLng;
		zoom?: number;
		style?: string;
		children?: Snippet;
	}>();

	let bounds: LngLatBounds | undefined = $state();
</script>

<div class="map-container">
	<MapLibre {style} {center} {zoom} class="map" bind:bounds={bounds} onmoveend={() => handleMoveEnd(bounds)} standardControls>
		{#if children}
			{@render children()}
		{/if}

		{#if $markers}
			<GeoJSON data={$markers}>
				<CircleLayer
					id="tree-canopies"
					filter={["!", ["in", ["get", "state"], ["literal", ["stump", "gone"]]]]}
					paint={{
						"circle-color": [
							"match",
							["get", "state"],
							"stump", "#000000",
							"gone", "#000000",
							"unknown", "#ffd700",
							"dead", "#8b4513",
							"#228b22", // default
						],
						"circle-opacity": 0.5,
						"circle-radius": 20,
					}}
				/>

				<CircleLayer
					id="tree-trunks"
					paint={{
						"circle-color": "#000000",
						"circle-opacity": 0.5,
						"circle-radius": 5,
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
