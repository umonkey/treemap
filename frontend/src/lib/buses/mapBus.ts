import type { ILatLng } from '$lib/types';
import mitt from 'mitt';

type MapBusEvent = {
	center: ILatLng;
	select: string;
	menu: string;
	fit: {
		start: ILatLng;
		end: ILatLng;
	};
	onMoved: {
		lat: number;
		lon: number;
		zoom: number;
	};
};

export const mapBus = mitt<MapBusEvent>();
