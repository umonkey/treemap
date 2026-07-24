<script lang="ts">
	import { MapLibre, GeoJSON, LineLayer, AttributionControl } from 'svelte-maplibre';
	import 'maplibre-gl/dist/maplibre-gl.css';
	import { componentState } from './TrackPreview.svelte.ts';

	const { panoramaId }: { panoramaId: string } = $props();

	$effect(() => {
		componentState.reload(panoramaId);
	});
</script>

<div class="track-preview">
	<h3>GPS Track Preview</h3>
	{#if componentState.loading}
		<p aria-busy="true">Loading GPX track...</p>
	{:else if componentState.error}
		<p class="error">{componentState.error}</p>
	{:else if componentState.trackData}
		<div class="map-wrapper">
			<MapLibre
				style={componentState.layer}
				class="map"
				center={[44.5152, 40.1872]}
				zoom={13}
				attributionControl={false}
			>
				<AttributionControl compact={true} position="bottom-left" />
				<GeoJSON data={componentState.trackData}>
					<LineLayer
						paint={{
							'line-color': '#007aff',
							'line-width': 4
						}}
					/>
				</GeoJSON>
			</MapLibre>
		</div>
	{/if}
</div>

<style>
	.track-preview {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.map-wrapper {
		width: 100%;
		height: 400px;
		border-radius: 8px;
		overflow: hidden;
		border: 1px solid var(--pico-muted-border-color, #ccc);
		position: relative;
	}

	:global(.map) {
		width: 100%;
		height: 100%;
	}

	.error {
		color: var(--color-error, red);
	}
</style>
