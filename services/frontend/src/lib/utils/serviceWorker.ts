import { browser, dev } from '$app/environment';

let pending: Promise<ServiceWorkerRegistration | undefined> | undefined;

/**
 * Register the service worker once per page load.
 *
 * Registration is best effort: some embedded browsers and locked-down profiles refuse it, and the
 * app remains fully functional without it (only the precached offline fallback and the upload
 * reminders are lost). A refusal therefore resolves as `undefined` instead of being rethrown, so it
 * never surfaces as an unhandled rejection.
 */
export function registerServiceWorker(): Promise<ServiceWorkerRegistration | undefined> {
	if (dev || !browser || !('serviceWorker' in navigator)) {
		return Promise.resolve(undefined);
	}

	pending ??= new Promise((resolve) => {
		const register = () => {
			navigator.serviceWorker.register('/service-worker.js').then(resolve, (error) => {
				console.warn(
					'[sw] registration failed; offline cache and upload reminders are unavailable',
					error
				);
				resolve(undefined);
			});
		};

		// Wait for the load event so registration does not compete with the initial asset fetches.
		if (document.readyState === 'complete') {
			register();
		} else {
			window.addEventListener('load', register, { once: true });
		}
	});

	return pending;
}
