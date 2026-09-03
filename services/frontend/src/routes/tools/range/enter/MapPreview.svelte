<script lang="ts">
	import { MapLibre, GeoJSON, LineLayer, FillLayer, Marker } from 'svelte-maplibre';
	import 'maplibre-gl/dist/maplibre-gl.css';
	import TreeLayer from '$lib/components/map/TreeLayer.svelte';
	import { RangeMapPreviewState, type IGcpWithRadius } from './MapPreview.svelte.ts';
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

		{#each gcps as gcp}
			{#if !Number.isNaN(gcp.lat) && !Number.isNaN(gcp.lng) && !(gcp.lat === 0 && gcp.lng === 0)}
				<Marker lngLat={[gcp.lng, gcp.lat]}>
					<div class="gcp-marker">
						<span>{gcp.index}</span>
					</div>
				</Marker>

				{#if gcp.radius > 0}
					{@const gj = state.getCircleGeoJson(gcp)}
					{#if gj}
						<GeoJSON data={gj}>
							<FillLayer
								paint={{
									'fill-color': '#0172ad',
									'fill-opacity': 0.15
								}}
							/>
							<LineLayer
								paint={{
									'line-color': '#0172ad',
									'line-width': 2,
									'line-opacity': 0.7
								}}
							/>
						</GeoJSON>
					{/if}
				{/if}
			{/if}
		{/each}

		{#each trees as tree}
			{#if !Number.isNaN(tree.lat) && !Number.isNaN(tree.lng)}
				<Marker lngLat={[tree.lng, tree.lat]}>
					<div class="recorded-tree-marker" title="Recorded Tree"></div>
				</Marker>
			{/if}
		{/each}

		{#if operatorPos && !Number.isNaN(operatorPos.lat) && !Number.isNaN(operatorPos.lng)}
			<Marker lngLat={[operatorPos.lng, operatorPos.lat]}>
				<div class="operator-marker" title="Operator Location"></div>
			</Marker>
		{/if}

		{#if suggestedLocation && !Number.isNaN(suggestedLocation.lat) && !Number.isNaN(suggestedLocation.lng)}
			<Marker lngLat={[suggestedLocation.lng, suggestedLocation.lat]}>
				<div class="suggested-tree-marker" title="Suggested Tree Location"></div>
			</Marker>
		{/if}
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

	.gcp-marker {
		width: 24px;
		height: 24px;
		background-color: #000;
		color: #fff;
		border: 2px solid #fff;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 12px;
		font-weight: bold;
		box-shadow: 0 0 4px rgba(0, 0, 0, 0.5);
	}

	.operator-marker {
		width: 16px;
		height: 16px;
		background-color: #525f7a;
		border: 2px solid #fff;
		border-radius: 50%;
		box-shadow: 0 0 4px rgba(0, 0, 0, 0.5);
	}

	.suggested-tree-marker {
		width: 22px;
		height: 22px;
		background-color: #2e7d32;
		border: 3px solid #fff;
		border-radius: 50%;
		box-shadow: 0 0 6px rgba(0, 0, 0, 0.6);
	}

	.recorded-tree-marker {
		width: 18px;
		height: 18px;
		background-color: #1b5e20;
		border: 2px solid #fff;
		border-radius: 50%;
		box-shadow: 0 0 5px rgba(0, 0, 0, 0.5);
	}
</style>
