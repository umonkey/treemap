import { mapBus } from '$lib/buses';
import type { ILatLng, MountFn, DestroyFn } from '$lib/types';

type MoveFn = (ll: ILatLng) => void;

export const hooks = (mount: MountFn, destroy: DestroyFn, onMove: MoveFn) => {
	const handleCenter = ({ lat, lon }: { lat: number; lon: number }) => {
		console.debug(`[LocationPicker] Center updated: ${lat},${lon}`);
		onMove({ lat, lng: lon });
	};

	mount(() => mapBus.on('onMoved', handleCenter));
	destroy(() => mapBus.off('onMoved', handleCenter));
};
