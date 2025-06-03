import { ls } from '$lib/utils/localStorage';
import { writable } from 'svelte/store';

const STORE_KEY = 'clusterStore';

export const clusterStore = writable<boolean>(ls.read(STORE_KEY) ?? false);

clusterStore.subscribe((value: boolean) => {
	ls.write(STORE_KEY, value);
});
