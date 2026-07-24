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
			{#if componentState.trackData}
				<GeoJSON data={componentState.geoJson}>
					<LineLayer
						paint={{
							'line-color': '#007aff',
							'line-width': 4
						}}
					/>
				</GeoJSON>
			{/if}
		</MapLibre>
	</div>
</div>

<style>
	.track-preview {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		height: 100%;
	}

	.map-wrapper {
		width: 100%;
		height: 100%;
		aspect-ratio: 1;
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
