import mitt from "mitt";

type SoundBusEvent = {
	correct: undefined;
	wrong: undefined;
	finished: undefined;
};

export const soundBus = mitt<SoundBusEvent>();
