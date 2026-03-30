<script lang="ts">
	import { onMount } from 'svelte';
	import { CircleLayer, GeoJSON } from 'svelte-maplibre';
	import { treeLayerState } from './TreeLayer.svelte.ts';

	onMount(treeLayerState.onMount);
</script>

{#if treeLayerState.markers}
	<GeoJSON data={treeLayerState.markers}>
		<CircleLayer
			id="tree-crowns"
			paint={{
				'circle-color': [
					'match',
					['get', 'state'],
					['stump', 'gone'],
					'#000000',
					'unknown',
					'#ffd700',
					'dead',
					'#8b4513',
					'#228B22'
				],
				'circle-radius': [
					'interpolate',
					['exponential', 2],
					['zoom'],
					10,
					['*', ['get', 'crown'], 0.00428],
					22,
					['*', ['get', 'crown'], 17.534]
				],
				'circle-opacity': ['match', ['get', 'state'], ['stump', 'gone'], 0.2, 0.5],
				'circle-pitch-alignment': 'map',
				'circle-pitch-scale': 'map'
			}}
			onclick={treeLayerState.handleClick}
		/>

		<CircleLayer
			id="tree-trunks"
			paint={{
				'circle-color': '#000000',
				'circle-radius': [
					'interpolate',
					['exponential', 2],
					['zoom'],
					10,
					['*', ['get', 'trunk'], 0.00428],
					22,
					['*', ['get', 'trunk'], 17.534]
				],
				'circle-opacity': 0.8,
				'circle-pitch-alignment': 'map',
				'circle-pitch-scale': 'map'
			}}
		/>
	</GeoJSON>
{/if}
