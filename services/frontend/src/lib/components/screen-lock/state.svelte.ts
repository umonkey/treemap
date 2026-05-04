import { settingsStore } from '$lib/stores/settingsStore';
import { releaseWakeLock, requestWakeLock } from './utils';

class WakeLockState {
	private keepAwake = $state(false);

	constructor() {
		settingsStore.subscribe((s) => {
			this.keepAwake = s.keepAwake;
		});
	}

	get enabled() {
		return this.keepAwake;
	}

	set enabled(value: boolean) {
		settingsStore.update((s) => ({ ...s, keepAwake: value }));
	}

	async update() {
		if (this.keepAwake) {
			await requestWakeLock();
		} else {
			await releaseWakeLock();
		}
	}

	async handleVisibilityChange() {
		if (
			this.keepAwake &&
			typeof document !== 'undefined' &&
			document.visibilityState === 'visible'
		) {
			await requestWakeLock();
		}
	}
}

export const wakeLockState = new WakeLockState();
