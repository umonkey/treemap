import { writable, derived } from 'svelte/store';
import { DEFAULT_MAP_CENTER } from '$lib/constants';

export const mapState = writable({
	center: DEFAULT_MAP_CENTER,
	zoom: 15
});

export const mapCenter = derived(mapState, ($mapState) => $mapState.center);
export const mapZoom = derived(mapState, ($mapState) => $mapState.zoom);
