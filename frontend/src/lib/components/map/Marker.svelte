<script lang="ts">
	import type { LngLat } from 'maplibre-gl';
	import { Marker } from 'svelte-maplibre';
	import ICON from '$lib/assets/marker-icon-2x.png';

	const { center } = $props<{
		center: LngLat;
	}>();
</script>

<Marker lngLat={center} offset={[0, -21]}>
	<img src={ICON} alt="You are here" />
</Marker>

<style>
	img {
		width: 25px;
		height: 41px;
	}
</style>
