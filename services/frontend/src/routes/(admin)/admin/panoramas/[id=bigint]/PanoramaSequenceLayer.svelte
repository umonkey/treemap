<script lang="ts">
	import { CircleLayer, GeoJSON, LineLayer } from 'svelte-maplibre';
	import { PanoramaSequenceLayerState } from './PanoramaSequenceLayer.svelte.ts';

	const {
		panoramaId,
		minzoom = 18,
		selectedImageId,
		onSelectImage
	}: {
		panoramaId: string;
		minzoom?: number;
		selectedImageId?: string;
		onSelectImage: (id: string) => void;
	} = $props();

	const componentState = new PanoramaSequenceLayerState();

	$effect(() => {
		componentState.onSelectImage = onSelectImage;
		componentState.selectedImageId = selectedImageId;
	});

	$effect(() => {
		const cleanup = componentState.init();
		return cleanup;
	});

	$effect(() => {
		componentState.reload(panoramaId);
	});
</script>

{#if componentState.geoJsonData}
	<GeoJSON data={componentState.geoJsonData}>
		<LineLayer
			filter={['==', ['get', 'kind'], 'sequence']}
			paint={{ 'line-color': '#007aff', 'line-width': 4 }}
		/>
		<CircleLayer
			{minzoom}
			filter={['==', ['get', 'kind'], 'image']}
			onclick={componentState.handleCircleClick}
			paint={{
				'circle-color': '#007aff',
				'circle-radius': 5,
				'circle-stroke-width': 1,
				'circle-stroke-color': '#ffffff'
			}}
		/>
		{#if selectedImageId}
			<CircleLayer
				{minzoom}
				filter={['==', ['get', 'id'], selectedImageId]}
				paint={{
					'circle-color': '#007aff',
					'circle-radius': 10,
					'circle-opacity': 0.5,
					'circle-stroke-width': 2,
					'circle-stroke-color': '#ffffff',
					'circle-stroke-opacity': 0.8
				}}
			/>
		{/if}
	</GeoJSON>
{/if}
