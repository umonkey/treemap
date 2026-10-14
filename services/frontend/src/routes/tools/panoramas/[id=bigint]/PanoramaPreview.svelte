<script lang="ts">
	import { AttributionControl, MapLibre } from 'svelte-maplibre';
	import 'maplibre-gl/dist/maplibre-gl.css';
	import PanoramaViewer from '$lib/components/panoramas/PanoramaViewer.svelte';
	import MapRays from '$lib/components/map/MapRays.svelte';
	import FullScreenIcon from '$lib/icons/FullScreenIcon.svelte';
	import { PanoramaPreviewState } from './PanoramaPreview.svelte.ts';
	import PanoramaHintsLayer from './PanoramaHintsLayer.svelte';
	import PanoramaSequenceLayer from './PanoramaSequenceLayer.svelte';
	import TrackLayer from './TrackLayer.svelte';

	const { panoramaId, minzoom = 18 }: { panoramaId: string; ratio?: string; minzoom?: number } =
		$props();

	const componentState = new PanoramaPreviewState();

	let fullscreenElement = $state<HTMLElement | null>(null);

	const handleSelectImage = (id: string) => componentState.selectImage(id);

	$effect(() => {
		componentState.reload(panoramaId);
	});

	$effect(() => {
		const cleanup = componentState.init();
		return cleanup;
	});
</script>

<div class="panorama-preview" bind:this={fullscreenElement}>
	<div class="panes-container">
		<div class="map-wrapper">
			<div class="top-left">
				<button
					type="button"
					class="control fullscreen"
					onclick={() => componentState.toggleFullscreen(fullscreenElement)}
					aria-label="Fullscreen"
				>
					<FullScreenIcon />
				</button>
			</div>
			<MapLibre
				style={componentState.layer}
				bind:map={componentState.map}
				class="map"
				center={[44.5152, 40.1872]}
				zoom={13}
				onload={componentState.handleMapLoad}
				attributionControl={false}
			>
				<AttributionControl compact={true} position="bottom-left" />
				<TrackLayer {panoramaId} />
				<MapRays length={20} />
				<PanoramaSequenceLayer
					{panoramaId}
					{minzoom}
					selectedImageId={componentState.selectedImageId}
					onSelectImage={handleSelectImage}
				/>
				<PanoramaHintsLayer {panoramaId} />
			</MapLibre>
		</div>

		<div class="viewer-wrapper">
			{#if componentState.selectedImage}
				<PanoramaViewer
					image={componentState.selectedImage}
					angle={componentState.yaw}
					onMove={(angle) => componentState.handleViewerMove(angle)}
					canEdit={true}
					canHide={true}
					canFullScreen={false}
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

		&:fullscreen {
			width: 100vw;
			height: 100vh;
			padding: 0;
			box-sizing: border-box;

			.panes-container {
				height: 100%;
				gap: 2px;
			}

			.map-wrapper,
			.viewer-wrapper {
				height: 100%;
				aspect-ratio: auto;
			}
		}
	}

	.top-left {
		position: absolute;
		top: 10px;
		left: 10px;
		display: flex;
		flex-direction: column;
		background-color: white;
		color: black;
		border-radius: 4px;
		overflow: hidden;
		box-shadow: 0 0 0 2px rgba(0, 0, 0, 0.1);
		z-index: 1;
	}

	.control {
		width: 29px;
		height: 29px;
		cursor: pointer;
		background-color: transparent;
		border: none;
		color: inherit;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0;

		&:hover {
			background-color: rgba(0, 0, 0, 0.05);
		}

		:global(svg) {
			width: 20px;
			height: 20px;
		}
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
