<script lang="ts">
	import { MapLibre, FillLayer, GeoJSON } from 'svelte-maplibre';
	import type { LngLatBounds } from 'maplibre-gl';
	import 'maplibre-gl/dist/maplibre-gl.css';
	import type { ILatLng } from '$lib/types';
	import { onMount, type Snippet } from 'svelte';
	import { mapState } from './MapLibre.svelte.ts';

	// import { MAPTILER_KEY } from '$lib/env';
	// style = `https://api.maptiler.com/maps/openstreetmap/style.json?key=${MAPTILER_KEY}`,

	const {
		center,
		zoom = 13,
		style = 'https://basemaps.cartocdn.com/gl/positron-gl-style/style.json',
		children = undefined
	} = $props<{
		center: ILatLng;
		zoom?: number;
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
		{zoom}
		class="map"
		bind:bounds
		onmovestart={() => mapState.handleMoveStart()}
		onmoveend={() => mapState.handleMoveEnd(bounds)}
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
