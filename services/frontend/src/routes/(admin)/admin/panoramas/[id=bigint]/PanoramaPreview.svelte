<script lang="ts">
	import { AttributionControl, GeoJSON, LineLayer, MapLibre } from 'svelte-maplibre';
	import 'maplibre-gl/dist/maplibre-gl.css';
	import PanoramaViewer from '$lib/components/panoramas/PanoramaViewer.svelte';
	import MapRays from '$lib/components/map/MapRays.svelte';
	import { PanoramaPreviewState } from './PanoramaPreview.svelte.ts';
	import PanoramaSequenceLayer from './PanoramaSequenceLayer.svelte';

	const { panoramaId, minzoom = 18 }: { panoramaId: string; ratio?: string; minzoom?: number } =
		$props();

	const componentState = new PanoramaPreviewState();

	const handleSelectImage = (id: string) => componentState.selectImage(id);

	$effect(() => {
		const cleanup = componentState.init();
		return cleanup;
	});

	$effect(() => {
		componentState.reload(panoramaId);
	});
</script>

<div class="panorama-preview">
	<div class="panes-container">
		<div class="map-wrapper">
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
				<MapRays length={20} />
				<PanoramaSequenceLayer
					{panoramaId}
					{minzoom}
					selectedImageId={componentState.selectedImageId}
					onSelectImage={handleSelectImage}
				/>
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
			</MapLibre>
		</div>
		<div class="viewer-wrapper">
			{#if componentState.selectedImage}
				<PanoramaViewer
					image={componentState.selectedImage}
					angle={componentState.yaw}
					onMove={(angle) => componentState.handleViewerMove(angle)}
				/>
			{:else}
				<div class="placeholder">
					<p aria-busy={componentState.loadingImage}>
						{componentState.loadingImage ? 'Loading image...' : 'Select an image on the map'}
					</p>
				</div>
			{/if}
		</div>
	</div>
</div>

<style>
	.panorama-preview {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		width: 100%;
	}

	.panes-container {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
		width: 100%;
	}

	.map-wrapper,
	.viewer-wrapper {
		width: 100%;
		aspect-ratio: 1 / 1;
		overflow: hidden;
		border: none;
		position: relative;
		background-color: var(--pico-card-background-color, #fff);
	}

	.placeholder {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 100%;
		height: 100%;
		color: var(--pico-muted-color, #666);
	}

	:global(.map),
	:global(.map canvas),
	:global(.map-wrapper) {
		width: 100%;
		height: 100%;
		cursor: crosshair !important;
	}
</style>
