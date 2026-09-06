<script lang="ts">
	import { onMount } from 'svelte';
	import { CircleLayer, GeoJSON, LineLayer } from 'svelte-maplibre';
	import { PanoramicLayerLogic } from './PanoramicLayer.svelte.ts';
	import { mapState } from './MapLibre.svelte.ts';
	import { mapPoiStore } from '$lib/stores/mapPoi.svelte';

	const componentState = new PanoramicLayerLogic();

	onMount(componentState.onMount);

	$effect(() => {
		if (mapState.panoramasLayer) {
			if (!componentState.data) {
				componentState.reload();
			}
		} else {
			mapPoiStore.panoramas = [];
		}
	});
</script>

{#if mapState.panoramasLayer && componentState.data}
	<GeoJSON data={componentState.data}>
		<LineLayer
			id="panoramas-sequences"
			filter={['==', ['get', 'kind'], 'sequence']}
			paint={{
				'line-color': '#007aff',
				'line-width': 4,
				'line-opacity': 0.5
			}}
		/>

		<CircleLayer
			id="panoramas-images"
			minzoom={18}
			filter={['==', ['get', 'kind'], 'image']}
			paint={{
				'circle-color': '#007aff',
				'circle-radius': ['step', ['zoom'], 4, 19, 10],
				'circle-opacity': 0.8,
				'circle-stroke-width': 1,
				'circle-stroke-color': '#ffffff'
			}}
			onclick={componentState.handleClick}
		/>
	</GeoJSON>
{/if}
