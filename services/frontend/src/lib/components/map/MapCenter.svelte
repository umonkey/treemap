<script lang="ts">
	import { onMount } from 'svelte';
	import { mapBus } from '$lib/buses/mapBus';
	import CrossHair from '$lib/icons/CrossHair.svelte';
	import { mapState } from './MapLibre.svelte.ts';
	import { getMapContext } from 'svelte-maplibre';

	const mapContext = getMapContext();

	onMount(() => {
		const update = () => {
			const map = mapContext?.map || mapState.map;
			if (map) {
				const center = map.getCenter();
				mapBus.emit('center', { lat: center.lat, lng: center.lng });
			}
		};

		const map = mapContext?.map || mapState.map;
		map?.on('move', update);
		update();

		return () => {
			const m = mapContext?.map || mapState.map;
			m?.off('move', update);
		};
	});
</script>

<div class="center">
	<CrossHair />
</div>

<style>
	.center {
		position: absolute;
		left: 50%;
		top: 50%;
		z-index: 10;
		transform: translate(-50%, -50%);
		width: 50px;
		height: 50px;
	}
</style>
