<script lang="ts">
	import ICON from '$lib/assets/marker-icon-2x.png';
	import type { LngLatLike } from 'maplibre-gl';
	import { Marker } from 'svelte-maplibre';

	const { center } = $props<{
		center: LngLatLike;
	}>();
</script>

<Marker lngLat={center} offset={[0, -16]}>
	<img src={ICON} alt="You are here" />
</Marker>

<style>
	img {
		width: 25px;
		height: 41px;
	}
</style>
