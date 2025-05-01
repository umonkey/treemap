import mitt from 'mitt';

type MapBusEvent = {
	center: [number, number];
	select: string;
};

export const mapBus = mitt<MapBusEvent>();
