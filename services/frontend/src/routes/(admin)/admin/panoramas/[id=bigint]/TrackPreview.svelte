<script lang="ts">
	import { MapLibre, GeoJSON, LineLayer, AttributionControl, Marker } from 'svelte-maplibre';
	import 'maplibre-gl/dist/maplibre-gl.css';
	import { TrackPreviewState } from './TrackPreview.svelte.ts';

	const {
		panoramaId,
		offset = 0,
		ratio = '2 / 1'
	}: { panoramaId: string; offset?: number; ratio?: string } = $props();

	const componentState = new TrackPreviewState();

	const currentPoint = $derived(componentState.getCoordinates(offset));

	$effect(() => {
		componentState.reload(panoramaId);
	});
</script>

<div class="track-preview">
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
			{#if currentPoint}
				<Marker lngLat={[currentPoint.lng, currentPoint.lat]} offset={[0, 0]}>
					<div class="current-location-dot"></div>
				</Marker>
			{/if}
		</MapLibre>
	</div>
</div>

<style>
	.track-preview {
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

	.current-location-dot {
		width: 16px;
		height: 16px;
		background-color: #ff4500;
		border: 2px solid #fff;
		border-radius: 50%;
		box-shadow: 0 0 4px rgba(0, 0, 0, 0.4);
	}
</style>
