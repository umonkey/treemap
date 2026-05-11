import { browser } from '$app/environment';

const SYNC_TAG = 'upload-check';
const PERIODIC_SYNC_TAG = 'upload-reminder';
const PERIODIC_SYNC_INTERVAL = 60 * 60 * 1000; // 1 hour

/**
 * Initialize background reminders for pending uploads.
 * This requests notification permissions and registers background sync tasks.
 */
export async function initBackgroundReminders() {
	if (!browser || !('serviceWorker' in navigator) || !('Notification' in window)) {
		return;
	}

	const registration = await navigator.serviceWorker.ready;

	// Request permission if not already granted
	if (Notification.permission === 'default') {
		const permission = await Notification.requestPermission();
		if (permission !== 'granted') {
			console.debug('[notifications] Permission not granted.');
			return;
		}
	}

	if (Notification.permission !== 'granted') {
		return;
	}

	// Register background sync (triggered when online)
	if ('sync' in registration) {
		try {
			await (registration as any).sync.register(SYNC_TAG);
			console.debug('[notifications] Background sync registered.');
		} catch (e) {
			console.error('[notifications] Failed to register background sync:', e);
		}
	}

	// Register periodic background sync (triggered periodically by the system)
	if ('periodicSync' in registration) {
		try {
			const status = await (navigator as any).permissions.query({
				name: 'periodic-background-sync'
			});

			if (status.state === 'granted') {
				await (registration as any).periodicSync.register(PERIODIC_SYNC_TAG, {
					minInterval: PERIODIC_SYNC_INTERVAL
				});
				console.debug('[notifications] Periodic background sync registered.');
			} else {
				console.debug('[notifications] Periodic background sync permission not granted.');
			}
		} catch (e) {
			console.error('[notifications] Failed to register periodic background sync:', e);
		}
	}
}
