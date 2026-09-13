<script lang="ts">
	import { GeoJSON, LineLayer, Marker } from 'svelte-maplibre';
	import { componentState } from './NearestTree.svelte.ts';

	const {
		distance = 100,
		label = true,
		count = 2
	}: {
		distance?: number;
		label?: boolean;
		count?: number;
	} = $props();

	$effect(() => {
		componentState.maxDistance = distance;
	});

	$effect(() => {
		componentState.count = count;
	});
</script>

{#each componentState.nearestTrees as tree (tree.poi.url)}
	<GeoJSON data={tree.line}>
		<LineLayer
			layout={{ 'line-cap': 'round', 'line-join': 'round' }}
			paint={{
				'line-color': '#888888',
				'line-width': 2,
				'line-dasharray': [2, 2]
			}}
		/>
	</GeoJSON>

	{#if label}
		<Marker lngLat={tree.midpoint}>
			<div class="distance-label">
				{tree.distance.toFixed(1)}m
			</div>
		</Marker>
	{/if}
{/each}

<style>
	.distance-label {
		background: white;
		color: black;
		padding: 2px 6px;
		border-radius: 4px;
		font-size: 12px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
		pointer-events: none;
		white-space: nowrap;
	}
</style>
