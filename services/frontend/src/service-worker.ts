/// <reference types="@sveltejs/kit" />
/// <reference lib="webworker" />
import { build, files, version } from '$service-worker';

const CACHE = `cache-${version}`;
const ASSETS = [...build, ...files];

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
			const cachedResponse = await cache.match(url.pathname);
			if (cachedResponse) return cachedResponse;
		}

		// For everything else, or if the asset wasn't in cache, try the network.
		try {
			return await fetch(fetchEvent.request);
		} catch (error) {
			// Fallback to cache if network fails (e.g. for the index page).
			const cachedResponse = await cache.match(fetchEvent.request);
			if (cachedResponse) return cachedResponse;

			// Return 503 Service Unavailable if truly offline and not in cache.
			return new Response('Offline', { status: 503 });
		}
	}

	fetchEvent.respondWith(respond());
});
