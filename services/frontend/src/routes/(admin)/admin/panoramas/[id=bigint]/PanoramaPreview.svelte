<script lang="ts">
	import { MapLibre, GeoJSON, LineLayer, CircleLayer, AttributionControl } from 'svelte-maplibre';
	import 'maplibre-gl/dist/maplibre-gl.css';
	import { PanoramaPreviewState } from './PanoramaPreview.svelte.ts';

	const {
		panoramaId,
		ratio = '2 / 1',
		minzoom = 18
	}: { panoramaId: string; ratio?: string; minzoom?: number } = $props();

	const componentState = new PanoramaPreviewState();

	$effect(() => {
		componentState.reload(panoramaId);
	});
</script>

<div class="panorama-preview">
	<div class="map-wrapper" style:aspect-ratio={ratio}>
		<MapLibre
			style={componentState.layer}
			bind:map={componentState.map}
			class="map"
			center={[44.5152, 40.1872]}
			zoom={13}
			onload={componentState.fitBounds}
			attributionControl={false}
		>
			<AttributionControl compact={true} position="bottom-left" />
			{#if componentState.geoJsonData}
				<GeoJSON data={componentState.geoJsonData}>
					<LineLayer
						filter={['==', ['get', 'kind'], 'sequence']}
						paint={{
							'line-color': '#007aff',
							'line-width': 4
						}}
					/>
					<CircleLayer
						{minzoom}
						filter={['==', ['get', 'kind'], 'image']}
						paint={{
							'circle-color': '#007aff',
							'circle-radius': 5,
							'circle-stroke-width': 1,
							'circle-stroke-color': '#ffffff'
						}}
					/>
				</GeoJSON>
			{/if}
		</MapLibre>
	</div>
</div>

<style>
	.panorama-preview {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		width: 100%;
	}

	.map-wrapper {
		width: 100%;
		border-radius: 8px;
		overflow: hidden;
		border: 1px solid var(--pico-muted-border-color, #ccc);
		position: relative;
	}

	:global(.map) {
		width: 100%;
		height: 100%;
	}
</style>
