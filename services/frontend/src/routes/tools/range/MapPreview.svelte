<script lang="ts">
	import { MapLibre } from 'svelte-maplibre';
	import 'maplibre-gl/dist/maplibre-gl.css';
	import TreeLayer from '$lib/components/map/TreeLayer.svelte';
	import MapCenter from '$lib/components/map/MapCenter.svelte';
	import GcpLayer from './GcpLayer.svelte';
	import { RangeMapPreviewState, type IGcpWithRadius } from './MapPreview.svelte.ts';

	const {
		gcps
	}: {
		gcps: IGcpWithRadius[];
	} = $props();

	const state = new RangeMapPreviewState();
</script>

<div class="map-preview-wrapper">
	<MapLibre
		style={state.layer}
		bind:map={state.map}
		class="map"
		center={[44.5152, 40.1872]}
		zoom={14}
		onload={() => state.fitBounds(gcps)}
		attributionControl={false}
	>
		<TreeLayer />
		<GcpLayer {gcps} />
		<MapCenter />
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
