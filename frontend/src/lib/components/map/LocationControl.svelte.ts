import type { IMyPosition } from '$lib/types';

export class LocationState {
	public position = $state<IMyPosition | null>(null);

	private watchId: number | null = null;

	public start = () => {
		if (!('geolocation' in navigator)) {
			console.warn('[GEO] Geolocation is not available, not tracking.');
			return;
		}

		if (this.watchId !== null) {
			console.debug(`[GEO] Already tracking, watch=${this.watchId}.`);
			return;
		}

		this.watchId = navigator.geolocation.watchPosition(
			(position) => {
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
					console.debug(`[GEO] My position updated: ${pos.lat},${pos.lng} ~ ${pos.accuracy}`);
				}
			},
			(error) => {
				console.error(`[GEO] Error ${error.code}: ${error.message}`, error);

				if (error.code === 1) {
					this.stop(); // access denied, stop tracking
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
			console.debug('[GEO] Tracking stopped.');
		}
	};

	public onMount = () => {
		window.addEventListener('focus', this.start);
		window.addEventListener('visibilitychange', this.start);

		this.start();

		return () => {
			window.removeEventListener('focus', this.start);
			window.removeEventListener('visibilitychange', this.start);
			this.stop();
			console.debug('[GEO] LocationState destroyed.');
		};
	};
}
