<script lang="ts">
	import { Marker } from 'svelte-maplibre';
	import { PointsLayerLogic } from './PointsLayer.svelte.ts';
	import type { ILatLng } from '$lib/types';
	import type { ITriangulatedTree } from '../store.svelte';
	import { rangeStore } from '../store.svelte';

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
		{@const isSelected = tree.id === rangeStore.selectedTree}
		<Marker lngLat={[tree.lng, tree.lat]}>
			<button
				type="button"
				class="recorded-tree-marker {isSelected ? 'selected' : ''}"
				title="Tree #{i + 1}"
				onclick={() => rangeStore.setSelectedTree(tree.id)}
			>
				<span>{i + 1}</span>
			</button>
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
		width: 28px;
		height: 28px;
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
		cursor: pointer;
		padding: 0;
		transition:
			transform 0.2s,
			box-shadow 0.2s;

		&:hover {
			transform: scale(1.1);
		}

		&.selected {
			background-color: #d32f2f;
			border: 3px solid #ffeb3b;
			box-shadow: 0 0 8px rgba(211, 47, 47, 0.8);
		}
	}
</style>
