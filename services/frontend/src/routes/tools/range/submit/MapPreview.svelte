<script lang="ts">
	import { MapLibre } from 'svelte-maplibre';
	import 'maplibre-gl/dist/maplibre-gl.css';
	import TreeLayer from '$lib/components/map/TreeLayer.svelte';
	import PointsLayer from '../enter/PointsLayer.svelte';
	import { SubmitMapPreviewState } from './MapPreview.svelte.ts';
	import type { ITriangulatedTree } from '../store.svelte';

	const {
		trees
	}: {
		trees: ITriangulatedTree[];
	} = $props();

	const state = new SubmitMapPreviewState();
</script>

<div class="map-preview-wrapper">
	<MapLibre
		style={state.layer}
		bind:map={state.map}
		class="map"
		center={state.getMapCenter(trees)}
		zoom={14}
		onload={() => state.fitBounds(trees)}
		attributionControl={false}
	>
		<TreeLayer />
		<PointsLayer {trees} />
	</MapLibre>
</div>

<style>
	.map-preview-wrapper {
		width: 100%;
		height: 350px;
		border-radius: 8px;
		overflow: hidden;
		border: 1px solid var(--pico-muted-border-color, #ccc);
		position: relative;
	}

	:global(.map) {
		width: 100%;
		height: 100%;
	}
</style>
