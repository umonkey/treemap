import { writable } from 'svelte/store';
import { DEFAULT_MODE } from '$lib/constants';
import { ls } from '$lib/utils/localStorage';

export const modeStore = writable<string>(ls.read('modeStore') || DEFAULT_MODE);

modeStore.subscribe((value: string) => {
	ls.write('modeStore', value);
});
