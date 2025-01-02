import mitt from 'mitt';

type LocationBusEvent = {
	start: void;
};

export const locationBus = mitt<LocationBusEvent>();
