<script lang="ts">
	import { Marker } from 'svelte-maplibre';
	import { GcpLayerLogic } from './GcpLayer.svelte.ts';
	import type { IGcpWithRadius } from './MapPreview.svelte.ts';

	const {
		gcps
	}: {
		gcps: IGcpWithRadius[];
	} = $props();

	const state = new GcpLayerLogic();
</script>

{#each gcps as gcp}
	{#if state.isValidGcp(gcp)}
		<Marker lngLat={[gcp.lng, gcp.lat]}>
			<div class="gcp-marker">
				<span>{gcp.label}</span>
			</div>
		</Marker>
	{/if}
{/each}

<style>
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
</style>
