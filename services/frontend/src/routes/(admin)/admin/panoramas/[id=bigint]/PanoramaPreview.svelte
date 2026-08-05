<script lang="ts">
	import { AttributionControl, CircleLayer, GeoJSON, LineLayer, MapLibre } from 'svelte-maplibre';
	import 'maplibre-gl/dist/maplibre-gl.css';
	import PanoramaViewer from '$lib/components/panoramas/PanoramaViewer.svelte';
	import MapRays from '$lib/components/map/MapRays.svelte';
	import CrossHair from '$lib/icons/CrossHair.svelte';
	import { PanoramaPreviewState } from './PanoramaPreview.svelte.ts';

	const { panoramaId, minzoom = 18 }: { panoramaId: string; ratio?: string; minzoom?: number } =
		$props();

	const componentState = new PanoramaPreviewState();

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
				onclick={(e) => componentState.handleMapClick(e)}
			>
				<AttributionControl compact={true} position="bottom-left" />
				<MapRays length={20} />
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
							onclick={(e) => componentState.handleCircleClick(e)}
							paint={{
								'circle-color': '#007aff',
								'circle-radius': 5,
								'circle-stroke-width': 1,
								'circle-stroke-color': '#ffffff'
							}}
						/>
						{#if componentState.selectedImageId}
							<CircleLayer
								{minzoom}
								filter={['==', ['get', 'id'], componentState.selectedImageId]}
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
			</MapLibre>
		</div>
		<div class="viewer-wrapper">
			{#if componentState.selectedImage}
				<PanoramaViewer
					image={componentState.selectedImage}
					angle={componentState.yaw}
					onMove={(angle) => componentState.handleViewerMove(angle)}
				/>
				<div class="crosshair">
					<CrossHair />
				</div>
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
		border-radius: 8px;
		overflow: hidden;
		border: 1px solid var(--pico-muted-border-color, #ccc);
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

	.crosshair {
		position: absolute;
		left: 50%;
		top: 50%;
		z-index: 10;
		transform: translate(-50%, -50%);
		width: 50px;
		height: 50px;
		pointer-events: none;
		color: white;
		filter: drop-shadow(0 0 2px rgba(0, 0, 0, 0.5));

		:global(svg) {
			width: 100%;
			height: 100%;
			fill: currentColor;
		}

		:global(.cls-1) {
			fill: currentColor;
		}
	}

	:global(.map),
	:global(.map canvas),
	:global(.map-wrapper) {
		width: 100%;
		height: 100%;
		cursor: crosshair !important;
	}
</style>
