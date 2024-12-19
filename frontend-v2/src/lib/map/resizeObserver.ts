import type { Map } from 'leaflet';

export const addResizeObserver = (map: Map) => {
	const resizeObserver = new ResizeObserver(() => {
		map.invalidateSize();
	});

	resizeObserver.observe(map.getContainer());
};
