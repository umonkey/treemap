import type { MountFn } from '$lib/types';
import { getMap } from '$lib/map';

export const hooks = (onMount: MountFn) => {
	const handleResize = () => {
		const map = getMap();
		console.debug('[map] Resize observer triggered, invalidating map size.');
		map.invalidateSize();
	};

	onMount(() => {
		try {
			const resizeObserver = new ResizeObserver(() => handleResize);
			resizeObserver.observe(getMap().getContainer());

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
