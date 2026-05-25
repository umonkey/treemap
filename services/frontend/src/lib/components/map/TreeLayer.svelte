<script lang="ts">
	import { onMount } from 'svelte';
	import { CircleLayer, GeoJSON } from 'svelte-maplibre';
	import { treeLayerState } from './TreeLayer.svelte.ts';

	onMount(treeLayerState.onMount);

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	const crownColor: any = [
		'match',
		['get', 'state'],
		['stump', 'gone'],
		'#000000',
		'unknown',
		'#ffd700',
		'dead',
		'#8b4513',
		'placeholder',
		'#808080',
		'#228B22'
	];
</script>

{#if treeLayerState.markers}
	<GeoJSON data={treeLayerState.markers}>
		<CircleLayer
			id="tree-crowns-small"
			maxzoom={15}
			paint={{
				'circle-color': crownColor,
				'circle-radius': treeLayerState.crownRadiusSmall,
				'circle-opacity': ['match', ['get', 'state'], ['stump', 'gone', 'placeholder'], 0.2, 0.5],
				'circle-pitch-alignment': 'map',
				'circle-pitch-scale': 'map'
			}}
			onclick={treeLayerState.handleClick}
			oncontextmenu={treeLayerState.handleContextMenu}
		/>

		<CircleLayer
			id="tree-crowns-large"
			minzoom={15}
			paint={{
				'circle-color': crownColor,
				'circle-radius': treeLayerState.crownRadiusLarge,
				'circle-opacity': ['match', ['get', 'state'], ['stump', 'gone', 'placeholder'], 0.2, 0.5],
				'circle-pitch-alignment': 'map',
				'circle-pitch-scale': 'map'
			}}
			onclick={treeLayerState.handleClick}
			oncontextmenu={treeLayerState.handleContextMenu}
		/>

		<CircleLayer
			id="tree-trunks"
			minzoom={15}
			filter={['>', ['get', 'trunk'], 0]}
			paint={{
				'circle-color': '#000000',
				'circle-radius': treeLayerState.trunkRadiusLarge,
				'circle-opacity': 0.8,
				'circle-pitch-alignment': 'map',
				'circle-pitch-scale': 'map'
			}}
		/>
	</GeoJSON>
{/if}
