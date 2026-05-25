import { updated } from '$app/stores';
import { toast } from 'svelte-sonner';

class CheckUpdatesButtonLogic {
	isChecking = $state<boolean>(false);
	private unsubscribe: (() => void) | undefined;

	constructor() {
		// Pure constructor: no side effects
	}

	/**
	 * Initialize the subscription to the updated store.
	 * SvelteKit requires at least one subscription to 'updated' for .check() to work.
	 */
	init = () => {
		if (this.unsubscribe) return;
		this.unsubscribe = updated.subscribe(() => {
			// We don't need to do anything with the value, just stay subscribed.
		});
	};

	destroy = () => {
		if (this.unsubscribe) {
			this.unsubscribe();
			this.unsubscribe = undefined;
		}
	};

	checkUpdates = async () => {
		if (this.isChecking) return;
		this.isChecking = true;

		try {
			console.debug('[pwa] Checking for updates...');
			const hasUpdate = await updated.check();

			if (hasUpdate) {
				console.info('[pwa] Update detected.');
				const registration = await navigator.serviceWorker.getRegistration();
				if (registration?.waiting) {
					registration.waiting.postMessage({ type: 'SKIP_WAITING' });
				} else {
					// Fallback: if no waiting worker, reload might trigger a check/update of SW
					window.location.reload();
				}
			} else {
				toast.success('You are using the latest version.');
			}
		} catch (err) {
			console.error('[pwa] Failed to check for updates:', err);
			toast.error('Failed to check for updates. Please try again later.');
		} finally {
			this.isChecking = false;
		}
	};
}

export const componentState = new CheckUpdatesButtonLogic();
