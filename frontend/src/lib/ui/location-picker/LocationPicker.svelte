<script lang="ts">
	import { Map, MapCenter, MapPin } from '$lib/ui';
	import type { ILatLng } from '$lib/types';
	import { onMount, onDestroy } from 'svelte';
	import { hooks } from './hooks';

	const { center, pin, onMove } = $props<{
		center: ILatLng;
		pin?: ILatLng;
		onMove: (ll: ILatLng) => void;
	}>();

	hooks(onMount, onDestroy, onMove);
</script>

<div class="mapContainer">
	<Map {center} zoom={19} crosshair={true}>
		<MapCenter />

		{#if pin}
			<MapPin center={pin} />
		{/if}
	</Map>
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
