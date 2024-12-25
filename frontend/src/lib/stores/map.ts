import { writable, derived } from 'svelte/store';
import { DEFAULT_MAP_CENTER } from '$lib/constants';
import { ls } from '$lib/utils/localStorage';

export const mapState = writable(
	ls.read('mapState') || {
		center: DEFAULT_MAP_CENTER,
		zoom: 15
	}
);

mapState.subscribe((value) => {
	ls.write('mapState', value);
});

export const mapCenter = derived(mapState, ($mapState) => $mapState.center);
export const mapZoom = derived(mapState, ($mapState) => $mapState.zoom);
