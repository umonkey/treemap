<script lang="ts">
	import { onMount } from 'svelte';
	import { CircleLayer, GeoJSON, LineLayer } from 'svelte-maplibre';
	import { panoramicLayerState } from './PanoramicLayer.svelte.ts';

	onMount(panoramicLayerState.onMount);
</script>

{#if panoramicLayerState.data}
	<GeoJSON data={panoramicLayerState.data}>
		<LineLayer
			id="mapillary-sequences"
			filter={['==', ['get', 'kind'], 'sequence']}
			paint={{
				'line-color': '#007aff',
				'line-width': 4,
				'line-opacity': 0.5
			}}
		/>

		<CircleLayer
			id="mapillary-images"
			minzoom={18}
			filter={['==', ['get', 'kind'], 'image']}
			paint={{
				'circle-color': '#007aff',
				'circle-radius': 4,
				'circle-opacity': 0.8,
				'circle-stroke-width': 1,
				'circle-stroke-color': '#ffffff'
			}}
			onclick={panoramicLayerState.handleClick}
		/>
	</GeoJSON>
{/if}
