<script lang="ts">
	import { onMount } from 'svelte';
	import { GeoJSON, LineLayer } from 'svelte-maplibre';
	import { TreeHintsLayerLogic } from './TreeHintsLayer.svelte.ts';
	import { mapState } from './MapLibre.svelte.ts';

	const componentState = new TreeHintsLayerLogic();

	onMount(componentState.onMount);

	$effect(() => {
		if (mapState.treeHintsLayer && !componentState.data) {
			componentState.reload();
		}
	});
</script>

{#if mapState.treeHintsLayer && componentState.data}
	<GeoJSON data={componentState.data}>
		<LineLayer
			id="tree-hints"
			paint={{
				'line-color': '#22c55e',
				'line-width': 2,
				'line-opacity': 0.6
			}}
		/>
	</GeoJSON>
{/if}
