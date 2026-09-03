<script lang="ts">
	import { MapLibre } from 'svelte-maplibre';
	import 'maplibre-gl/dist/maplibre-gl.css';
	import TreeLayer from '$lib/components/map/TreeLayer.svelte';
	import LocationTracker from '$lib/components/map/LocationTracker.svelte';
	import LocateButton from '$lib/components/map/LocateButton.svelte';
	import GcpLayer from '../GcpLayer.svelte';
	import CircleLayer from './CircleLayer.svelte';
	import PointsLayer from './PointsLayer.svelte';
	import { RangeMapPreviewState, type IGcpWithRadius } from './MapPreview.svelte.ts';
	import type { ILatLng } from '$lib/types';
	import type { ITriangulatedTree } from '../store.svelte';

	const {
		gcps,
		suggestedLocation,
		trees = []
	}: {
		gcps: IGcpWithRadius[];
		suggestedLocation?: ILatLng | null;
		trees?: ITriangulatedTree[];
	} = $props();

	const state = new RangeMapPreviewState();
	const mapCenter = $derived(state.getMapCenter(gcps));

	$effect(() => {
		state.fitBounds(gcps, trees, suggestedLocation);
	});
</script>

<div class="map-preview-wrapper">
	<MapLibre
		style={state.layer}
		bind:map={state.map}
		class="map"
		center={mapCenter}
		zoom={14}
		onload={() => state.fitBounds(gcps, trees, suggestedLocation)}
		attributionControl={false}
	>
		<TreeLayer />
		<CircleLayer {gcps} />
		<GcpLayer {gcps} />
		<PointsLayer {trees} {suggestedLocation} />

		<LocateButton />
		<LocationTracker />
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
