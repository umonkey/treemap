<script lang="ts">
	import { onMount } from 'svelte';
	import { mapBus } from '$lib/buses/mapBus';
	import CrossHair from '$lib/icons/CrossHair.svelte';
	import { mapState } from './MapLibre.svelte.ts';

	onMount(() => {
		const update = () => {
			if (mapState.map) {
				const center = mapState.map.getCenter();
				mapBus.emit('center', { lat: center.lat, lng: center.lng });
			}
		};

		mapState.map?.on('move', update);
		return () => {
			mapState.map?.off('move', update);
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
