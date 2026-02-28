import { writable } from 'svelte/store';
import { ls } from '$lib/utils/localStorage';
import { getPendingCount } from '$lib/db';

const AUTOUPLOAD_KEY = 'autoUpload';

type UploadStore = {
	pending: number;
	autoupload: boolean;
};

export const uploadStore = writable<UploadStore>({
	pending: 0,
	autoupload: ls.read(AUTOUPLOAD_KEY) ?? true
});

uploadStore.subscribe((value: UploadStore) => {
	ls.write(AUTOUPLOAD_KEY, value.autoupload);
});

// Initialize the store from the database.
getPendingCount().then((count) => {
	uploadStore.update((store) => {
		store.pending = count;
		return store;
	});
});

export const resetUploadCount = () => {
	uploadStore.update((store) => {
		store.pending = 0;
		return store;
	});
};

export const incrementUploadCount = () => {
	uploadStore.update((store) => {
		store.pending = store.pending + 1;
		return store;
	});
};

export const decrementUploadCount = () => {
	uploadStore.update((store) => {
		store.pending = Math.max(0, store.pending - 1);
		return store;
	});
};
