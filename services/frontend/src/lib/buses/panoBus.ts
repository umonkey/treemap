import mitt from 'mitt';

type PanoBusEvent = {
	// Reload when tree hints were added or updated in a panorama.
	reload: void;
	// Reload only hint-derived data after an image hint is added/deleted.
	reloadHints: void;
};

export const panoBus = mitt<PanoBusEvent>();
