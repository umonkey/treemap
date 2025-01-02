<script lang="ts">
	// This component implements location tracking via the Geolocation API.
	// It uses the `navigator.geolocation.watchPosition` method to track the user's location
	// and report it to other components via the locationStore.
	//
	// The process should be started using the locationBus message.  The idea is that once started,
	// the process continues even when the user navigates away from the map, so when they're back
	// to another map, the location is already available and up-to-date.

	import { locationBus } from '$lib/buses/locationBus';
	import { onMount } from 'svelte';
	import type { IMyPosition } from '$lib/types';
	import { locationStore } from '$lib/stores/locationStore';

	let lastPosition = $state<IMyPosition | null>(null);
	let watchId = $state<number | null>(null);

	onMount(() => {
		locationBus.on('start', () => {
			if (!('geolocation' in navigator)) {
				console.warning('[GEO] Geolocation is not available, not tracking.');
				return;
			}

			if (watchId !== null) {
				console.debug(`[GEO] Already tracking, watch=${watchId}.`);
				return;
			}

			watchId = navigator.geolocation.watchPosition(
				(position) => {
					let pos = {
						lat: position.coords.latitude,
						lng: position.coords.longitude,
						accuracy: position.coords.accuracy
					};

					if (
						lastPosition === null ||
						pos.lat !== lastPosition.lat ||
						pos.lng !== lastPosition.lng ||
						pos.accuracy !== lastPosition.accuracy
					) {
						lastPosition = pos;

						console.debug(`[GEO] My position updated: ${pos.lat},${pos.lng} ~ ${pos.accuracy}`);

						locationStore.update(() => {
							return pos;
						});
					}
				},
				(error) => {
					console.error(`[GEO] Error ${error.code}: ${error.message}`);

					if (error.code === 1) {
						watchId = null; // access denied, try to restart

						locationStore.update(() => {
							return null;
						});
					}
				},
				{
					enableHighAccuracy: true,
					maximumAge: 0
				}
			);

			console.debug(`[GEO] Tracking started, watch=${watchId}.`);
		});
	});
</script>
