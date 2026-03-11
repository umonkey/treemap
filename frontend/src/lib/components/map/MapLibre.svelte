<script lang="ts">
	import { MapLibre } from 'svelte-maplibre';
	import 'maplibre-gl/dist/maplibre-gl.css';
	import { MAPTILER_KEY } from '$lib/env';
	import type { ILatLng } from '$lib/types';
	import type { Snippet } from 'svelte';

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
</script>

<div class="map-container">
	<MapLibre {style} {center} {zoom} class="map" standardControls>
		{#if children}
			{@render children()}
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
