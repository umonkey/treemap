import mitt from 'mitt';

type LocationBusEvent = {
	start: undefined;
};

export const locationBus = mitt<LocationBusEvent>();
