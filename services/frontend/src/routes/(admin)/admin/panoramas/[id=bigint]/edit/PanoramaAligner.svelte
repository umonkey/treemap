<script lang="ts">
	import { onMount } from 'svelte';
	import { MapLibre, AttributionControl } from 'svelte-maplibre';
	import 'maplibre-gl/dist/maplibre-gl.css';
	import PanoramaAlignmentLayer from './PanoramaAlignmentLayer.svelte';
	import { PanoramaAlignerLogic } from './PanoramaAligner.svelte.ts';

	let {
		panoramaId,
		latOffset = $bindable(0),
		lonOffset = $bindable(0)
	}: {
		panoramaId: string;
		latOffset: number;
		lonOffset: number;
	} = $props();

	const componentState = new PanoramaAlignerLogic();

	$effect(() => {
		componentState.reload(panoramaId);
	});

	$effect(() => {
		latOffset = componentState.latOffset;
		lonOffset = componentState.lonOffset;
	});

	$effect(() => {
		if (latOffset !== componentState.latOffset && componentState.panorama) {
			componentState.latOffset = latOffset;
		}
		if (lonOffset !== componentState.lonOffset && componentState.panorama) {
			componentState.lonOffset = lonOffset;
		}
	});

	onMount(() => {
		window.addEventListener('keydown', componentState.handleKeydown);
		return () => {
			window.removeEventListener('keydown', componentState.handleKeydown);
		};
	});
</script>

<div class="panorama-aligner-container">
	<div class="map-wrapper">
		<MapLibre
			style="https://basemaps.cartocdn.com/gl/positron-gl-style/style.json"
			bind:map={componentState.map}
			center={[44.5152, 40.1872]}
			zoom={17}
			onload={componentState.fitBounds}
			class="map"
			attributionControl={false}
		>
			<AttributionControl compact={true} position="bottom-left" />
			{#if componentState.geoJson || componentState.hintsGeoJson}
				<PanoramaAlignmentLayer
					geoJson={componentState.geoJson}
					hintsGeoJson={componentState.hintsGeoJson}
					latOffset={componentState.latOffset}
					lonOffset={componentState.lonOffset}
					baseLatOffset={componentState.baseLatOffset}
					baseLonOffset={componentState.baseLonOffset}
				/>
			{/if}
		</MapLibre>

		<div class="nudge-controls">
			<button
				type="button"
				class="nudge-btn up"
				onclick={() => componentState.nudge('up')}
				title="Move North (10cm)">▲</button
			>
			<div class="nudge-middle">
				<button
					type="button"
					class="nudge-btn left"
					onclick={() => componentState.nudge('left')}
					title="Move West (10cm)">◀</button
				>
				<button
					type="button"
					class="nudge-btn right"
					onclick={() => componentState.nudge('right')}
					title="Move East (10cm)">▶</button
				>
			</div>
			<button
				type="button"
				class="nudge-btn down"
				onclick={() => componentState.nudge('down')}
				title="Move South (10cm)">▼</button
			>
		</div>
	</div>
</div>

<style>
	.panorama-aligner-container {
		display: flex;
		flex-direction: column;
		width: 100%;
		margin-bottom: var(--pico-spacing);
	}

	.map-wrapper {
		position: relative;
		width: 100%;
		height: 450px;
		border-radius: 8px;
		overflow: hidden;
		border: 1px solid var(--pico-muted-border-color, #ccc);
	}

	:global(.map) {
		width: 100%;
		height: 100%;
	}

	.nudge-controls {
		position: absolute;
		bottom: 15px;
		right: 15px;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 4px;
		z-index: 10;
		background: rgba(255, 255, 255, 0.9);
		padding: 8px;
		border-radius: 8px;
		box-shadow: 0 2px 6px rgba(0, 0, 0, 0.15);
	}

	.nudge-middle {
		display: flex;
		gap: 20px;
	}

	.nudge-btn {
		background: #007aff;
		color: white;
		border: none;
		border-radius: 4px;
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		cursor: pointer;
		font-size: 1rem;
		padding: 0;
		margin: 0;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
		transition: background 0.2s;
	}

	.nudge-btn:hover {
		background: #0056b3;
	}

	.nudge-btn:active {
		transform: scale(0.95);
	}
</style>
