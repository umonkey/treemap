import mitt from 'mitt';

type PanoBusEvent = {
	// Reload when tree hints were added or updated in a panorama.
	reload: void;
};

export const panoBus = mitt<PanoBusEvent>();
