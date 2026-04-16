import { derived, writable } from 'svelte/store';

type IStore = {
	files: File[];
	uploading: boolean;
	progress: number;
	message: string | null;
};

export const fileStore = writable<IStore>({
	files: [],
	uploading: false,
	progress: 0,
	message: null
});

export const storedFiles = derived(fileStore, (fileStore) => fileStore.files);
export const isUploading = derived(fileStore, (fileStore) => fileStore.uploading);
export const uploadProgress = derived(fileStore, (fileStore) => fileStore.progress);
export const uploadMessage = derived(fileStore, (fileStore) => fileStore.message);
