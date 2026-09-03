<script lang="ts">
	import { GeoJSON, FillLayer, LineLayer } from 'svelte-maplibre';
	import { CircleLayerLogic } from './CircleLayer.svelte.ts';
	import type { IGcpWithRadius } from './MapPreview.svelte.ts';

	const {
		gcps
	}: {
		gcps: IGcpWithRadius[];
	} = $props();

	const state = new CircleLayerLogic();
</script>

{#each gcps as gcp}
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
{/each}
