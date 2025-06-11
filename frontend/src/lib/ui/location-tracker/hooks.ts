import { get, writable } from 'svelte/store';
import { locationStore } from '$lib/stores/locationStore';
import type { MountFn, IMyPosition } from '$lib/types';

export const hooks = ({ onMount }: { onMount: MountFn }) => {
	const lastPosition = writable<IMyPosition | null>(null);

	const watchId = writable<number | null>(null);

	const handleStart = () => {
		if (!('geolocation' in navigator)) {
			console.warn('[GEO] Geolocation is not available, not tracking.');
			return;
		}

		if (get(watchId) !== null) {
			console.debug(`[GEO] Already tracking, watch=${get(watchId)}.`);
			return;
		}

		watchId.set(
			navigator.geolocation.watchPosition(
				(position) => {
					const pos = {
						lat: position.coords.latitude,
						lng: position.coords.longitude,
						accuracy: position.coords.accuracy
					};

					if (
						lastPosition === null ||
						pos.lat !== get(lastPosition)?.lat ||
						pos.lng !== get(lastPosition)?.lng ||
						pos.accuracy !== get(lastPosition)?.accuracy
					) {
						lastPosition.set(pos);

						console.debug(`[GEO] My position updated: ${pos.lat},${pos.lng} ~ ${pos.accuracy}`);

						// Make the new location available to other components.
						locationStore.update(() => {
							return pos;
						});
					} else {
						// console.debug(`[GEO] My position unchanged: ${pos.lat},${pos.lng} ~ ${pos.accuracy}`);
					}
				},
				(error) => {
					console.error(`[GEO] Error ${error.code}: ${error.message}`);

					if (error.code === 1) {
						watchId.set(null); // access denied, try to restart
						locationStore.set(null);
					}
				},
				{
					enableHighAccuracy: true,
					maximumAge: 0
				}
			)
		);

		console.debug(`[GEO] Tracking started, watch=${get(watchId)}.`);
	};

	onMount(() => {
		window.addEventListener('focus', handleStart);
		window.addEventListener('visibilitychange', handleStart);
		console.debug('[GEO] LocationTracker mounted.');

		// Don't wait for a signal, start tracking immediately.
		handleStart();

		return () => {
			window.removeEventListener('focus', handleStart);
			window.removeEventListener('visibilitychange', handleStart);
			console.debug('[GEO] LocationTracker destroyed.');
		};
	});
};
