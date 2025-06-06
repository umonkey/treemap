import mitt from 'mitt';
import type { ILatLng } from '$lib/types';

type MapBusEvent = {
	center: ILatLng;
	select: string;
	closePreview: void;
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
