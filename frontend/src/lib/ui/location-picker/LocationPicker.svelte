<script lang="ts">
	import MapLibre from '$lib/components/map/MapLibre.svelte';
	import MapCenter from '$lib/components/map/MapCenter.svelte';
	import Marker from '$lib/components/map/Marker.svelte';
	import type { ILatLng } from '$lib/types';
	import { pickerState } from './LocationPicker.svelte.ts';

	const { center, pin, onMove } = $props<{
		center: ILatLng;
		pin?: ILatLng;
		onMove: (ll: ILatLng) => void;
	}>();

	$effect(() => {
		pickerState.onMove = onMove;
		pickerState.handleCenter(center);
	});
</script>

<div class="mapContainer">
	<MapLibre onMove={pickerState.handleMove}>
		<MapCenter />

		{#if pin}
			<Marker center={pin} />
		{/if}
	</MapLibre>
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
