<script lang="ts">
	import { onMount } from 'svelte';
	import { Marker } from 'svelte-maplibre';
	import { LocationState } from './LocationControl.svelte.ts';

	const locationState = new LocationState();

	onMount(locationState.onMount);
</script>

{#if locationState.position}
	<Marker lngLat={[locationState.position.lng, locationState.position.lat]} offset={[0, 0]}>
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
