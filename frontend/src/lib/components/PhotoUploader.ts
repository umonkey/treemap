/**
 * Implements the logic for the photo uploader component.
 *
 * The component lets the user select random files, take photos, etc.
 * The files are transparently uploaded to the server.
 */

import { get, writable } from 'svelte/store';
import type { MountFn } from '$lib/types';
import { addPhotoToUploadQueue } from '$lib/upload';
import { db } from '$lib/db';
import { liveQuery } from 'dexie';

export type Thumbnail = {
	file: Blob | File;
	busy: boolean;
	error: boolean;
};

type Props = {
	treeId: string;
	onChange: (files: number) => void;
	onMount: MountFn;
};

export const load = ({ treeId, onChange, onMount }: Props) => {
	// This is files received from the file picker.
	// We use them to create thumbnails, and to upload data to the backend.
	// We aren't reporting this to the parent component.
	const thumbnails = writable<Thumbnail[]>([]);

	const appendFile = (file: File) => {
		addPhotoToUploadQueue(treeId, file);
	};

	// Handle the change event from the file picker element.
	// We get a bunch of files, which we need to (1) store for thumbnails,
	// and (2) send to the backend.
	const handleChange = (event: Event) => {
		const files = (event.target as HTMLInputElement).files;

		if (files && files.length > 0) {
			for (let i = 0; i < files.length; i++) {
				const file = files[i];
				appendFile(file);
			}
		}
	};

	const handlePaste = async (event: ClipboardEvent) => {
		const items = event.clipboardData?.items;

		if (!items) return;

		for (const item of items) {
			if (item.kind === 'file') {
				const file = item.getAsFile();
				if (file && file.type.startsWith('image/')) {
					appendFile(file);
				}
			}
		}
	};

	onMount(() => {
		const query = liveQuery(() => db.uploads.where('tree_id').equals(treeId.toString()).toArray());

		const subscription = query.subscribe({
			next: (data) => {
				thumbnails.set(
					data.map((item) => ({
						file: item.image,
						busy: item.status === 'uploading',
						error: item.status === 'failed'
					}))
				);
				onChange(data.length);
			},
			error: (err) => console.error('Failed to load thumbnails:', err)
		});

		document.addEventListener('paste', handlePaste);

		return () => {
			subscription.unsubscribe();
			document.removeEventListener('paste', handlePaste);
		};
	});

	return { thumbnails, handleChange };
};
