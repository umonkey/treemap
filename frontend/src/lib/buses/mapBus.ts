import mitt from 'mitt';

type MapBusEvent = {
	center: [number, number];
	select: string;
	onMoved: {
		lat: number;
		lon: number;
		zoom: number;
	};
};

export const mapBus = mitt<MapBusEvent>();
