<script lang="ts">
	import ICON from '$lib/assets/marker-icon-2x.png';
	import { Marker } from 'svelte-maplibre';
	import { mapMarkerStore } from '$lib/stores/mapMarker.svelte';
</script>

{#if mapMarkerStore.center}
	<Marker lngLat={mapMarkerStore.center} offset={[0, -16]}>
		<img src={ICON} alt="You are here" />
	</Marker>
{/if}

<style>
	img {
		width: 25px;
		height: 41px;
	}
</style>
