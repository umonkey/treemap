import { writable } from 'svelte/store';

type UploadStore = {
	pending: number;
};

export const uploadStore = writable<UploadStore>({
	pending: 0
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
