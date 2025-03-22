import { derived, writable } from 'svelte/store';
import { DEFAULT_MODE } from '$lib/constants';
import { ls } from '$lib/utils/localStorage';
import { ModeEnum } from '$lib/enums';

export const modeStore = writable<string>(ls.read('modeStore') || DEFAULT_MODE);

modeStore.subscribe((value: string) => {
	ls.write('modeStore', value);
});

export const isMapperMode = derived(modeStore, ($modeStore) => $modeStore === ModeEnum.Mapper);
