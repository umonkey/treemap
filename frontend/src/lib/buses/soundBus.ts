// This bus is used in the Learning mode to trigger audio playback,
// because the audio elements are outside of the page component tree.

import mitt from 'mitt';

type SoundBusEvent = {
	correct: undefined;
	wrong: undefined;
	finished: undefined;
};

export const soundBus = mitt<SoundBusEvent>();
