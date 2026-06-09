<script lang="ts">
	import { onMount } from 'svelte';
	import { GeoJSON, LineLayer } from 'svelte-maplibre';
	import { treeHintsLayerState } from './TreeHintsLayer.svelte.ts';

	onMount(treeHintsLayerState.onMount);
</script>

{#if treeHintsLayerState.data}
	<GeoJSON data={treeHintsLayerState.data}>
		<LineLayer
			id="mapillary-tree-hints"
			paint={{
				'line-color': '#22c55e',
				'line-width': 2,
				'line-opacity': 0.6
			}}
		/>
	</GeoJSON>
{/if}
