<script lang="ts">
	import Map from '$lib/components/Map.svelte';
	import type { LatLng } from 'leaflet';

	const { center, marker, onMove } = $props<{
		center: [number, number];
		marker?: [number, number];
		onMove: (center: [number, number]) => void;
	}>();

	const handleMove = (center: LatLng) => {
		onMove([center.lat, center.lng]);
	};
</script>

<div class="mapContainer">
	<Map {center} {marker} zoom={19} crosshair={true} onMove={handleMove} />
</div>

<style>
	.mapContainer {
		background-color: var(--form-background);
		width: 100%;
		aspect-ratio: 4/3;
		margin: var(--gap) 0;
		position: relative;
	}
</style>
