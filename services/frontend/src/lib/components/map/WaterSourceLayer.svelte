<script lang="ts">
	import { onMount } from 'svelte';
	import { CircleLayer, GeoJSON } from 'svelte-maplibre';
	import { WaterSourceLayerLogic } from './WaterSourceLayer.svelte.ts';

	const componentState = new WaterSourceLayerLogic();

	onMount(componentState.onMount);
</script>

{#if componentState.markers}
	<GeoJSON data={componentState.markers}>
		<CircleLayer
			id="water-source-discs"
			filter={['==', ['get', 'status'], 'operational']}
			paint={{
				'circle-color': '#1a73e8',
				'circle-opacity': 0.25,
				'circle-radius': componentState.radius50m,
				'circle-pitch-alignment': 'map',
				'circle-pitch-scale': 'map'
			}}
		/>
		<CircleLayer
			id="water-source-dots"
			paint={{
				'circle-color': '#1a73e8',
				'circle-radius': 5,
				'circle-stroke-color': '#ffffff',
				'circle-stroke-width': 1
			}}
			onclick={componentState.handleClick}
		/>
	</GeoJSON>
{/if}
