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

	async function respond() {
		const url = new URL(fetchEvent.request.url);
		const cache = await caches.open(CACHE);

		// Always try the cache first for assets
		if (ASSETS.includes(url.pathname)) {
			const response = await cache.match(url.pathname);
			if (response) return response;
		}

		// For everything else, try the network
		try {
			const response = await fetch(fetchEvent.request);

			// Cache successful responses
			if (response.status === 200) {
				cache.put(fetchEvent.request, response.clone());
			}

			return response;
		} catch {
			// Fallback to cache if network fails
			const response = await cache.match(fetchEvent.request);
			if (response) return response;

			return new Response('Offline', { status: 408 });
		}
	}

	fetchEvent.respondWith(respond());
});
