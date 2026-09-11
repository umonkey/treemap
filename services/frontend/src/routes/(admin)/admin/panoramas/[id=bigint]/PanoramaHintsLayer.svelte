<script lang="ts">
	import { GeoJSON, LineLayer } from 'svelte-maplibre';
	import { PanoramaHintsLayerState } from './PanoramaHintsLayer.svelte.ts';

	const { panoramaId }: { panoramaId: string } = $props();

	const componentState = new PanoramaHintsLayerState();

	$effect(() => {
		const cleanup = componentState.init();
		return cleanup;
	});

	$effect(() => {
		componentState.reload(panoramaId);
	});
</script>

{#if componentState.hintsGeoJsonData}
	<GeoJSON data={componentState.hintsGeoJsonData}>
		<LineLayer
			filter={['==', ['get', 'kind'], 'hint']}
			paint={{
				'line-color': '#22c55e',
				'line-width': 2,
				'line-opacity': 0.8
			}}
		/>
	</GeoJSON>
{/if}
