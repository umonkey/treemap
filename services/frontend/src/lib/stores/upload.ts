import { writable } from 'svelte/store';
import { ls } from '$lib/utils/localStorage';
import { db } from '$lib/db';
import { updateBadge } from '$lib/utils/badges';
import { liveQuery } from 'dexie';

const AUTOUPLOAD_KEY = 'autoUpload';

type UploadStore = {
	pending: number;
	autoupload: boolean;
	uploading: boolean;
};

export const uploadStore = writable<UploadStore>({
	pending: 0,
	autoupload: ls.read(AUTOUPLOAD_KEY) ?? true,
	uploading: false
});

uploadStore.subscribe((value: UploadStore) => {
	ls.write(AUTOUPLOAD_KEY, value.autoupload);
	updateBadge(value.pending);
});

// Automatically synchronize the pending count with the database.
liveQuery(() => db.uploads.count()).subscribe((count) => {
	uploadStore.update((store) => {
		store.pending = count;
		return store;
	});
});

export const setUploading = (value: boolean) => {
	uploadStore.update((store) => {
		store.uploading = value;
		return store;
	});
};
