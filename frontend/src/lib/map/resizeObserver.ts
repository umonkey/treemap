import type { Map } from 'leaflet';

export const addResizeObserver = (map: Map) => {
	try {
		const resizeObserver = new ResizeObserver(() => {
			try {
				map.invalidateSize();
			} catch (e) {
				console.debug(`[map] Error invalidating state, map probably gone: ${e}`);
			}
		});

		resizeObserver.observe(map.getContainer());

		map.on('unload', () => {
			console.debug('[map] Disconnecting resize observer.');
			resizeObserver.disconnect();
		});
	} catch (e) {
		// This normally only happens in unit tests.
		console.warn(`[map] ResizeObserver not supported: ${e}`);
	}
};
