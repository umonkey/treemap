<script lang="ts">
	import type { ILatLng } from '$lib/types';
	import { onMount } from 'svelte';
	import { Map, MapRow, MapMyPosition, MapFullscreen } from '$lib/ui';
	import { hooks } from './hooks';

	const { start, end, count } = $props<{
		start: ILatLng;
		end: ILatLng;
		count: number;
	}>();

	const { handleBoundsChange } = hooks();

	$effect(() => handleBoundsChange(start, end));
</script>

<div class="container">
	<Map center={start} zoom={19}>
		<MapFullscreen />
		<MapMyPosition />
		<MapRow {start} {end} {count} />
	</Map>
</div>

<style>
	.container {
		background-color: var(--form-background);
		width: 100%;
		aspect-ratio: 4/3;
		margin: var(--gap) 0;
		position: relative;
	}
</style>
