import { locale } from '$lib/locale';
import { locationStore } from '$lib/stores/locationStore';
import type { IMyPosition } from '$lib/types';
import { showError } from '$lib/errors';

class LocationTracker {
	public position = $state<IMyPosition | null>(null);
	public isTracking = $state<boolean>(false);

	private watchId: number | null = null;

	public start = () => {
		if (!('geolocation' in navigator)) {
			console.warn('[GEO] Geolocation is not available, not tracking.');
			return;
		}

		if (this.watchId !== null) {
			// console.debug(`[GEO] Already tracking, watch=${this.watchId}.`);
			return;
		}

		this.watchId = navigator.geolocation.watchPosition(
			(position) => {
				this.isTracking = true;
				const pos = {
					lat: position.coords.latitude,
					lng: position.coords.longitude,
					accuracy: position.coords.accuracy
				};

				if (
					this.position === null ||
					pos.lat !== this.position.lat ||
					pos.lng !== this.position.lng ||
					pos.accuracy !== this.position.accuracy
				) {
					this.position = pos;
					locationStore.set(pos);
					console.debug(`[GEO] My position updated: ${pos.lat},${pos.lng} ~ ${pos.accuracy}`);
				}
			},
			(error) => {
				if (error.code === 1) {
					this.stop(); // access denied, stop tracking
					console.debug('[GEO] User denied access, stopping.');
					showError(locale.toastLocationDenied());
				} else {
					console.error(`[GEO] Error ${error.code}: ${error.message}`, error);
				}
			},
			{
				enableHighAccuracy: true,
				maximumAge: 0
			}
		);

		console.debug(`[GEO] Tracking started, watch=${this.watchId}.`);
	};

	public stop = () => {
		if (this.watchId !== null) {
			navigator.geolocation.clearWatch(this.watchId);
			this.watchId = null;
			this.isTracking = false;
			console.debug('[GEO] Tracking stopped.');
		}
	};

	public onMount = () => {
		window.addEventListener('focus', this.startIfGranted);
		window.addEventListener('visibilitychange', this.startIfGranted);

		this.startIfGranted();

		return () => {
			window.removeEventListener('focus', this.startIfGranted);
			window.removeEventListener('visibilitychange', this.startIfGranted);
			this.stop();
			console.debug('[GEO] LocationTracker destroyed.');
		};
	};

	private startIfGranted = () => {
		if (navigator.permissions && navigator.permissions.query) {
			navigator.permissions.query({ name: 'geolocation' }).then((result) => {
				if (result.state === 'granted') {
					this.start();
				}
			});
		}
	};
}

export const locationTracker = new LocationTracker();
