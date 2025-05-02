import { DEFAULT_MODE } from '$lib/constants';
import { ModeEnum } from '$lib/enums';
import { ls } from '$lib/utils/localStorage';
import { derived, writable } from 'svelte/store';

const STORAGE_KEY = 'modeStore';

export const modeStore = writable<string>(ls.read(STORAGE_KEY) || DEFAULT_MODE);

modeStore.subscribe((value: string) => {
	ls.write(STORAGE_KEY, value);
});

export const isMapperMode = derived(modeStore, ($modeStore) => $modeStore === ModeEnum.Mapper);
