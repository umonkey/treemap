import { locale } from '$lib/locale';
import { locationStore } from '$lib/stores/locationStore';
import type { IMyPosition } from '$lib/types';
import { showError, showInfo } from '$lib/errors';

class LocationTracker {
	public position = $state<IMyPosition | null>(null);
	public isTracking = $state<boolean>(false);

	private watchId: number | null = null;

	public start = () => {
		if (!('geolocation' in navigator)) {
			console.warn('[GEO] Geolocation is not available, not tracking.');
			showInfo(locale.toastLocationNotAvailable());
			return;
		}

		if (this.watchId !== null) {
			console.debug(`[GEO] Already tracking, watch=${this.watchId}.`);
			return;
		}

		this.watchId = navigator.geolocation.watchPosition(
			(position) => {
				if (!this.isTracking) {
					this.isTracking = true;
				}

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
				} else if (error.code === 2 || error.code === 3) {
					// Position unavailable or timed out. Expected on devices without a
					// reliable location provider (desktops have no GPS at all), and the
					// watch stays active afterwards, so this is not an application error.
					// Warn rather than error: `captureConsoleIntegration` only reports errors.
					console.warn(`[GEO] Error ${error.code}: ${error.message}`, error);
				} else {
					console.error(`[GEO] Error ${error.code}: ${error.message}`, error);
				}
			},
			{
				enableHighAccuracy: true,
				maximumAge: 0,
				timeout: 10000
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

	private startIfGranted = async () => {
		try {
			if (!navigator.permissions?.query) {
				console.debug('[GEO] Could not query permissions.');
				return;
			}

			const result = await navigator.permissions.query({ name: 'geolocation' });

			if (result?.state === 'granted') {
				this.start();
			} else {
				console.debug(`[GEO] Permissions are ${result?.state ?? 'unknown'}, not starting.`);
			}
		} catch (error) {
			// A privacy browser or extension can replace `navigator.permissions.query`
			// with a shim that calls the native function with the wrong receiver, which
			// throws "Illegal invocation"; other engines reject names they do not
			// enumerate. Neither is actionable, and an unhandled rejection is reported
			// by Sentry's global handler, so an unreadable state means "unknown" and
			// tracking simply does not auto-start.
			console.debug('[GEO] Could not query permissions:', error);
		}
	};
}

export const locationTracker = new LocationTracker();
