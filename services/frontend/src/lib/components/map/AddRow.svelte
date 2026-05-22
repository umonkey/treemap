<script lang="ts">
	import Icon from '$lib/icons/Ruler.svelte';
	import { onMount } from 'svelte';
	import { GeoJSON, LineLayer, CircleLayer } from 'svelte-maplibre';
	import { Control } from 'svelte-maplibre';
	import { addState } from './AddRow.svelte.ts';
	import { mapMode } from '$lib/stores/mapMode';

	onMount(addState.onMount);
</script>

<Control position="bottom-right">
	<div class="maplibregl-ctrl-group">
		<button type="button" onclick={addState.toggle} title="Add a new tree">
			<Icon />
		</button>
	</div>
</Control>

{#if $mapMode === 'add-row' && addState.previewData}
	<GeoJSON data={addState.previewData}>
		<LineLayer
			layout={{ 'line-cap': 'round', 'line-join': 'round' }}
			paint={{
				'line-color': '#007cbf',
				'line-width': 2,
				'line-opacity': 0.8
			}}
		/>

		<CircleLayer
			paint={{
				'circle-color': '#228B22',
				'circle-radius': [
					'interpolate',
					['exponential', 2],
					['zoom'],
					10,
					['*', 5, 0.00428],
					22,
					['*', 5, 17.534]
				],
				'circle-opacity': 0.5,
				'circle-pitch-alignment': 'map',
				'circle-pitch-scale': 'map'
			}}
		/>
	</GeoJSON>
{/if}

<style>
	button {
		padding: 4px;
		color: #000;
	}
</style>
