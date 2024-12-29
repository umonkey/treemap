import mitt from 'mitt';

type SoundBusEvent = {
	correct: void;
	wrong: void;
	finished: void;
};

export const soundBus = mitt<SoundBusEvent>();
