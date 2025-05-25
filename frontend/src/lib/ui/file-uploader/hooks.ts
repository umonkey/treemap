/**
 * Implements the logic for the file uploader component.
 *
 * The component lets the user select random files, take photos, etc.
 * The files are transparently uploaded to the server, ids are received.
 *
 * The upload ids are then reported to the parent component to be used
 * for creating photos, etc.
 *
 * TODO: needs proper error handling.
 */

import { get, writable } from 'svelte/store';
import { apiClient } from '$lib/api';

type Item = {
	file: File;
	uploading: boolean;
	error: boolean;
};

export const load = ({
	onBusy,
	onChange
}: {
	onBusy: (value: boolean) => void;
	onChange: (ids: string[]) => void;
}) => {
	// The hidden file picker component.
	const input = writable<HTMLInputElement | null>(null);

	// This is files received from the file picker.
	// We use them to create thumbnails, and to upload data to the backend.
	// We aren't reporting this to the parent component.
	const items = writable<Item[]>([]);

	// This is the list of successfully uploaded file ids.
	// We add ids as soon as the file is successfully uploaded.
	const uploads = writable<string[]>([]);

	// This flag indicates that we're in process of uploading and cannot be interrupted.
	// The flag is used by the parent to disable the submit button.
	const busy = writable<boolean>(false);

	// Send busy flag to the parent.
	busy.subscribe((value) => {
		onBusy(value);
	});

	// Send uploaded file ids to the parent.
	uploads.subscribe((value) => {
		onChange(value);
	});

	// Handle the change event from the file picker element.
	// We get a bunch of files, which we need to (1) store for thumbnails,
	// and (2) send to the backend.
	const handleChange = (event: Event) => {
		const files = (event.target as HTMLInputElement).files;

		if (files && files.length > 0) {
			for (let i = 0; i < files.length; i++) {
				const file = files[i];

				// The index of the current file in the list.
				// We'll use it to update the upload status.
				const idx = get(items).length;

				items.update((current) => [
					...current,
					{
						file,
						uploading: false,
						error: false
					}
				]);

				handleRetry(idx);
			}
		}

		console.debug(`[FileUploader] Have ${get(items).length} files selected.`);
	};

	// Retry file upload.
	// This is called by the display component when the user clicks on the retry button.
	const handleRetry = (idx: number) => {
		busy.set(true);

		const file = get(items)[idx].file;

		// Mark the file as uploading.
		items.update((current) => {
			const updated = [...current];
			updated[idx].uploading = true;
			updated[idx].error = false;
			return updated;
		});

		// Start the upload process.
		apiClient
			.uploadSingleFile(file)
			.then((res) => {
				if (res.status === 200 && res.data?.id) {
					// We got the id of the uploaded file.
					const uploadId = res.data.id;
					uploads.update((currentUploads) => [...currentUploads, uploadId]);

					console.info(`[FileUploader] File uploaded successfully: ${file.name}, id=${uploadId}`);

					// Update the UI status.
					items.update((current) => {
						const updated = [...current];
						updated[idx].uploading = false;
						updated[idx].error = false;
						return updated;
					});
				} else {
					console.error(`[FileUploader] Failed to upload file: ${file.name}`);

					// Update the UI status.
					items.update((current) => {
						const updated = [...current];
						updated[idx].uploading = false;
						updated[idx].error = true;
						return updated;
					});
				}
			})
			.catch((e) => {
				console.error(`[FileUploader] Error uploading file: ${file.name}`, e);

				// Update the UI status.
				items.update((current) => {
					const updated = [...current];
					updated[idx].uploading = false;
					updated[idx].error = false;
					return updated;
				});
			})
			.finally(() => {
				busy.set(false);
			});
	};

	return { input, items, uploads, handleChange, handleRetry };
};
