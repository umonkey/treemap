import mitt from 'mitt';
import type { ILatLng } from '$lib/types';

type MapBusEvent = {
	center: ILatLng;
	select: string;
	onMoved: {
		lat: number;
		lon: number;
		zoom: number;
	};
};

export const mapBus = mitt<MapBusEvent>();
