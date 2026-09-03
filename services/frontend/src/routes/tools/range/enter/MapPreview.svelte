<script lang="ts">
	import { MapLibre } from 'svelte-maplibre';
	import 'maplibre-gl/dist/maplibre-gl.css';
	import TreeLayer from '$lib/components/map/TreeLayer.svelte';
	import LocationTracker from '$lib/components/map/LocationTracker.svelte';
	import LocateButton from '$lib/components/map/LocateButton.svelte';
	import GcpLayer from '../GcpLayer.svelte';
	import CircleLayer from './CircleLayer.svelte';
	import PointsLayer from './PointsLayer.svelte';
	import { RangeMapPreviewState, type IGcpWithRadius } from './EnterMapPreview.svelte.ts';
	import type { ILatLng } from '$lib/types';
	import type { ITriangulatedTree } from '../store.svelte';
	import { locationStore } from '$lib/stores/locationStore';

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
	const operatorPos = $derived($locationStore);

	const validGcp = $derived(
		gcps.find(
			(g) => g && !Number.isNaN(g.lat) && !Number.isNaN(g.lng) && !(g.lat === 0 && g.lng === 0)
		)
	);
	const mapCenter = $derived(
		validGcp
			? ([validGcp.lng, validGcp.lat] as [number, number])
			: ([44.5152, 40.1872] as [number, number])
	);

	$effect(() => {
		state.fitBounds(gcps, suggestedLocation, operatorPos, trees);
	});
</script>

<div class="map-preview-wrapper">
	<MapLibre
		style={state.layer}
		bind:map={state.map}
		class="map"
		center={mapCenter}
		zoom={14}
		onload={() => state.fitBounds(gcps, suggestedLocation, operatorPos, trees)}
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
