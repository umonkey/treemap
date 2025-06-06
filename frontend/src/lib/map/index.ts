import type { Map } from 'leaflet';
export { mapHome } from './home';
export { getContext } from 'svelte';
import { getContext } from 'svelte';

// This is used for context access by Map plugins.
export const mapKey = Symbol();

export const getMap = (): Map => {
	const map = getContext<Map>(mapKey);

	if (!map) {
		throw new Error('Map context not found.');
	}

	return map;
};
