<script lang="ts">
	import { onMount } from 'svelte';
	import { Marker } from 'svelte-maplibre';
	import { locationTracker } from './LocationTracker.svelte.ts';

	onMount(locationTracker.onMount);
</script>

{#if locationTracker.position}
	<Marker lngLat={[locationTracker.position.lng, locationTracker.position.lat]} offset={[0, 0]}>
		<div class="user-location-dot"></div>
	</Marker>
{/if}

<style>
	.user-location-dot {
		width: 16px;
		height: 16px;
		background-color: #1083fe;
		border: 2px solid #fff;
		border-radius: 50%;
		box-shadow: 0 0 4px rgba(0, 0, 0, 0.4);
	}
</style>
