<script lang="ts">
	import { Marker } from 'svelte-maplibre';
	import { PointsLayerLogic } from './PointsLayer.svelte.ts';
	import type { ILatLng } from '$lib/types';
	import type { ITriangulatedTree } from '../store.svelte';

	const {
		trees = [],
		suggestedLocation
	}: {
		trees?: ITriangulatedTree[];
		suggestedLocation?: ILatLng | null;
	} = $props();

	const state = new PointsLayerLogic();
</script>

{#each trees as tree, i}
	{#if state.isValidPoint(tree)}
		<Marker lngLat={[tree.lng, tree.lat]}>
			<div class="recorded-tree-marker" title="Tree #{i + 1}">
				<span>{i + 1}</span>
			</div>
		</Marker>
	{/if}
{/each}

{#if state.isValidPoint(suggestedLocation)}
	<Marker lngLat={[suggestedLocation.lng, suggestedLocation.lat]}>
		<div class="suggested-tree-marker" title="Suggested Tree Location"></div>
	</Marker>
{/if}

<style>
	.suggested-tree-marker {
		width: 22px;
		height: 22px;
		background-color: #2e7d32;
		border: 3px solid #fff;
		border-radius: 50%;
		box-shadow: 0 0 6px rgba(0, 0, 0, 0.6);
	}

	.recorded-tree-marker {
		width: 24px;
		height: 24px;
		background-color: #1b5e20;
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
