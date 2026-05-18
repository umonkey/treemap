/// <reference types="@sveltejs/kit" />
/// <reference lib="webworker" />
import { build, files, version } from '$service-worker';
import { updateBadge } from '$lib/utils/badges';

const CACHE = `cache-${version}`;
const ASSETS = [...build, ...files];

interface PeriodicSyncEvent extends ExtendableEvent {
	tag: string;
}

interface NotificationClickEvent extends ExtendableEvent {
	notification: Notification;
	action: string;
}

interface NetworkInformation extends EventTarget {
	type?: 'bluetooth' | 'cellular' | 'ethernet' | 'none' | 'wifi' | 'wimax' | 'other' | 'unknown';
}

interface NavigatorWithConnection extends Navigator {
	connection?: NetworkInformation;
}

self.addEventListener('install', (event) => {
	const extendableEvent = event as ExtendableEvent;
	async function addFilesToCache() {
		const cache = await caches.open(CACHE);
		await cache.addAll(ASSETS);
	}

	extendableEvent.waitUntil(addFilesToCache());
});

self.addEventListener('activate', (event) => {
	const extendableEvent = event as ExtendableEvent;
	async function deleteOldCaches() {
		for (const key of await caches.keys()) {
			if (key !== CACHE) await caches.delete(key);
		}
	}

	extendableEvent.waitUntil(deleteOldCaches());
});

self.addEventListener('fetch', (event) => {
	const fetchEvent = event as FetchEvent;
	if (fetchEvent.request.method !== 'GET') return;

	const url = new URL(fetchEvent.request.url);

	// Only handle requests to our own origin to avoid breaking API calls.
	if (url.origin !== self.origin) return;

	async function respond() {
		const cache = await caches.open(CACHE);

		// Always try the cache first for assets and the index page.
		// SvelteKit's ASSETS includes files from the static directory and the build output.
		const isAsset = ASSETS.includes(url.pathname);
		const isIndex = url.pathname === '/';

		if (isAsset || isIndex) {
			const cacheKey = isIndex ? '/index.html' : url.pathname;
			const cachedResponse = await cache.match(cacheKey);
			if (cachedResponse) return cachedResponse;
		}

		// For everything else, or if the asset wasn't in cache, try the network.
		try {
			return await fetch(fetchEvent.request);
		} catch {
			// Fallback to cache if network fails (e.g. for the index page).
			const cachedResponse = await cache.match(fetchEvent.request);
			if (cachedResponse) return cachedResponse;

			// Return 503 Service Unavailable if truly offline and not in cache.
			return new Response('Offline', { status: 503 });
		}
	}

	fetchEvent.respondWith(respond());
});

/**
 * Get the number of pending uploads in IndexedDB.
 */
async function getPendingUploadsCount(): Promise<number> {
	return new Promise((resolve) => {
		const request = indexedDB.open('TreeMapUploads');
		request.onerror = () => resolve(0);
		request.onsuccess = () => {
			const db = request.result;
			if (!db.objectStoreNames.contains('uploads')) {
				resolve(0);
				return;
			}
			const transaction = db.transaction('uploads', 'readonly');
			const store = transaction.objectStore('uploads');
			const statusIndex = store.index('status');

			let count = 0;
			const cursorRequest = statusIndex.openCursor();
			cursorRequest.onsuccess = () => {
				const cursor = cursorRequest.result;
				if (cursor) {
					const item = cursor.value;
					if ((item.status === 'pending' || item.status === 'failed') && item.retry_count < 5) {
						count++;
					}
					cursor.continue();
				} else {
					resolve(count);
				}
			};
			cursorRequest.onerror = () => resolve(0);
		};
	});
}

/**
 * Show a notification if conditions are met.
 */
async function checkAndNotify() {
	const count = await getPendingUploadsCount();

	// Update app badge if supported.
	await updateBadge(count);

	// 1. Check connectivity.
	// In Chrome, we can check for wifi specifically.
	const conn = (navigator as NavigatorWithConnection).connection;
	const isWifi = conn ? conn.type === 'wifi' || conn.type === 'ethernet' : navigator.onLine;

	if (!isWifi) {
		return;
	}

	// 2. Check if we have anything to upload.
	if (count > 0) {
		const registration = (self as unknown as ServiceWorkerGlobalScope)
			.registration as ServiceWorkerRegistration;
		await registration.showNotification('Trees of Yerevan', {
			body: 'You have photos ready to upload. Connect to WiFi to finish.',
			icon: '/favicon.png', // Fallback icon
			badge: '/favicon.png',
			tag: 'upload-reminder',
			renotify: true,
			data: {
				url: '/profile/uploads'
			}
		});
	}
}

self.addEventListener('periodicsync', (event) => {
	const periodicEvent = event as PeriodicSyncEvent;
	if (periodicEvent.tag === 'upload-reminder') {
		periodicEvent.waitUntil(checkAndNotify());
	}
});

self.addEventListener('notificationclick', (event) => {
	const notificationEvent = event as NotificationClickEvent;
	notificationEvent.notification.close();

	notificationEvent.waitUntil(
		(self as unknown as ServiceWorkerGlobalScope).clients
			.matchAll({ type: 'window' })
			.then((clientList) => {
				const url = notificationEvent.notification.data?.url || '/';
				for (const client of clientList) {
					const windowClient = client as WindowClient;
					if (windowClient.url.endsWith(url) && 'focus' in windowClient) {
						return windowClient.focus();
					}
				}
				if ((self as unknown as ServiceWorkerGlobalScope).clients.openWindow) {
					return (self as unknown as ServiceWorkerGlobalScope).clients.openWindow(url);
				}
			})
	);
});
