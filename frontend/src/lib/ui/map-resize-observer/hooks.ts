import type { MountFn } from '$lib/types';
import { getMap } from '$lib/map';
import { get, writable } from 'svelte/store';
import { type Map } from 'leaflet';

export const hooks = (onMount: MountFn) => {
	const map = writable<Map | null>(null);

	const handleResize = () => {
		console.debug('[map] Resize observer triggered, invalidating map size.');
		get(map)?.invalidateSize();
	};

	onMount(() => {
		const m = getMap();
		map.set(m);

		try {
			const resizeObserver = new ResizeObserver(() => handleResize);
			resizeObserver.observe(m.getContainer());

			console.debug('[map] Resize observer connected.');

			return () => {
				console.debug('[map] Resize observer disconnected.');
				resizeObserver.disconnect();
			};
		} catch (e) {
			// This normally only happens in unit tests.
			console.warn(`[map] ResizeObserver not supported: ${e}`);
		}
	});
};
