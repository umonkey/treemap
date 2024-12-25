import { get } from 'svelte/store';
import { fileStore } from '$lib/stores/fileStore';
import { apiClient } from '$lib/api';

const uploadSingleFile = async (file: File, tree: string) => {
	await apiClient.uploadFile(tree, file);
	console.info(`[upload] Uploaded ${file.name}.`);
};

export const startUpload = async (tree: string) => {
	const count = get(fileStore).files.length;
	const files = get(fileStore).files;
	const retry = [];

	const totalSize = files.length;
	let uploadedSize = 0;

	console.debug(`[upload] Going to upload ${count} files for tree ${tree}.`);

	fileStore.update((store) => {
		store.message = null;
		return store;
	});

	try {
		fileStore.update((store) => {
			store.uploading = true;
			return store;
		});

		for (const file of files) {
			console.debug(`[upload] Uploading ${file.name}...`);

			try {
				await uploadSingleFile(file, tree);
			} catch (e) {
				console.error(`[upload] Failed to upload ${file.name}: ${e}`);
				retry.push(file);
			} finally {
				uploadedSize++;

				fileStore.update((store) => {
					store.progress = (uploadedSize / totalSize) * 100;
					return store;
				});
			}
		}
	} finally {
		console.debug('[upload] Finished.');

		fileStore.update((store) => {
			store.files = retry;
			store.uploading = false;
			return store;
		});
	}

	fileStore.update((store) => {
		store.message =
			'Finished uploading all photos. They will show up on the tree page shortly. You can upload more files if needed.';
		return store;
	});
};
