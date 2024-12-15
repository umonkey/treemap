import { writable, derived } from 'svelte/store';
import { ls } from '$lib/utils/localStorage';

export const baseLayerState = writable<string | undefined>(ls.read('baseLayerState'));

baseLayerState.subscribe((value) => {
	ls.write('baseLayerState', value);
});

export const baseLayer = derived(baseLayerState, ($mapState) => $mapState);
