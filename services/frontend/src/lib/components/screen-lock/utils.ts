/**
 * Screen Wake Lock API manager.
 * See: https://developer.mozilla.org/en-US/docs/Web/API/Screen_Wake_Lock_API
 */

// eslint-disable-next-line @typescript-eslint/no-explicit-any
let wakeLock: any = null;

export const isWakeLockSupported = (): boolean => {
	return typeof navigator !== 'undefined' && 'wakeLock' in navigator;
};

export const requestWakeLock = async () => {
	if (!isWakeLockSupported()) {
		return;
	}

	try {
		if (wakeLock) {
			return;
		}
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		wakeLock = await (navigator as any).wakeLock.request('screen');
		wakeLock.addEventListener('release', () => {
			console.debug('[wakelock] Released.');
			wakeLock = null;
		});
		console.debug('[wakelock] Acquired.');
	} catch (err) {
		console.debug('[wakelock] Failed to acquire:', err);
	}
};

export const releaseWakeLock = async () => {
	if (wakeLock) {
		try {
			await wakeLock.release();
		} catch (err) {
			console.debug('[wakelock] Failed to release:', err);
		}
		wakeLock = null;
	}
};
