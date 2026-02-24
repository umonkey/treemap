/**
 * Implements the logic for the photo uploader component.
 *
 * The component lets the user select random files, take photos, etc.
 * The files are transparently uploaded to the server.
 */

import { writable } from 'svelte/store';
import { addPhotoToUploadQueue } from '$lib/upload';

type Thumbnail = {
	file: File;
};

type Props = {
	treeId: string;
};

export const load = ({ treeId }: Props) => {
	// This is files received from the file picker.
	// We use them to create thumbnails, and to upload data to the backend.
	// We aren't reporting this to the parent component.
	const thumbnails = writable<Thumbnail[]>([]);

	// Handle the change event from the file picker element.
	// We get a bunch of files, which we need to (1) store for thumbnails,
	// and (2) send to the backend.
	const handleChange = (event: Event) => {
		const files = (event.target as HTMLInputElement).files;

		if (files && files.length > 0) {
			for (let i = 0; i < files.length; i++) {
				const file = files[i];

				thumbnails.update((current) => [
					...current,
					{
						file,
						uploading: false,
						error: false
					}
				]);

				addPhotoToUploadQueue(treeId, file);
			}
		}
	};

	return { thumbnails, handleChange };
};
