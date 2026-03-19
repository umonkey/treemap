import type { ILatLng } from '$lib/types';
import mitt from 'mitt';

type MapBusEvent = {
	// Triggered whenever a map is moved.
	// The clients can use this to update the center, e.g. when moving trees.
	center: ILatLng;

	pin: ILatLng | undefined;

	// Triggered by the map when a tree is clicked.
	select: string;

	// Triggered by the map container to show a preview.
	preview: string;

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
