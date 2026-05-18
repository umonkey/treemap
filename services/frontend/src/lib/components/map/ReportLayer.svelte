<script lang="ts">
	import { onMount } from 'svelte';
	import { CircleLayer, GeoJSON } from 'svelte-maplibre';
	import { reportLayerState } from './ReportLayer.svelte.ts';

	onMount(reportLayerState.onMount);
</script>

{#if reportLayerState.markers}
	<GeoJSON data={reportLayerState.markers}>
		<CircleLayer
			id="chatbot-reports"
			paint={{
				'circle-color': '#ff0000',
				'circle-radius': 25,
				'circle-opacity': 0.6,
				'circle-stroke-width': 2,
				'circle-stroke-color': '#ffffff'
			}}
			onclick={reportLayerState.handleClick}
		/>
	</GeoJSON>
{/if}
