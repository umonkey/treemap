import type { ILatLng } from '$lib/types';
import { mapBus } from '$lib/buses';

export const hooks = () => {
	const handleBoundsChange = (start: ILatLng, end: ILatLng) => {
		mapBus.emit('fit', { start, end });
	};

	return { handleBoundsChange };
};
