<script lang="ts">
	import { onMount } from 'svelte';
	import { CircleLayer, GeoJSON } from 'svelte-maplibre';
	import { alertLayerState } from './AlertLayer.svelte.ts';

	onMount(alertLayerState.onMount);
</script>

{#if alertLayerState.markers}
	<GeoJSON data={alertLayerState.markers}>
		<CircleLayer
			id="chatbot-alerts"
			paint={{
				'circle-color': '#ff0000',
				'circle-radius': 25,
				'circle-opacity': ['+', 0.1, ['*', ['get', 'weight'], 0.5]],
				'circle-stroke-width': 0
			}}
			onclick={alertLayerState.handleClick}
		/>
	</GeoJSON>
{/if}
