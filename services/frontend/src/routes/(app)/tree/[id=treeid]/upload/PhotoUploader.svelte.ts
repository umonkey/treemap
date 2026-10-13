import { db } from '$lib/db';
import { queuePhoto } from '$lib/upload';
import { liveQuery } from 'dexie';

export type Thumbnail = {
	file: Blob | File;
	src: string;
	busy: boolean;
	error: boolean;
};

export class PhotoUploader {
	thumbnails = $state<Thumbnail[]>([]);
	private treeId = '';
	private onChange: (files: number) => void = () => {};

	init = (treeId: string, onChange: (files: number) => void) => {
		this.treeId = treeId;
		this.onChange = onChange;

		const query = liveQuery(() => db.uploads.where('tree_id').equals(treeId.toString()).toArray());

		const subscription = query.subscribe({
			next: (data) => {
				// Cleanup existing URLs to avoid memory leaks.
				this.thumbnails.forEach((t) => {
					if (t.src) URL.revokeObjectURL(t.src);
				});

				this.thumbnails = data.map((item) => {
					const file = item.thumbnail || item.image;
					return {
						file,
						src: URL.createObjectURL(file),
						busy: item.status === 'uploading',
						error: item.status === 'failed'
					};
				});
				this.onChange(data.length);
			},
			error: (err) => console.error('Failed to load thumbnails:', err)
		});

		const onPaste = (event: ClipboardEvent) => this.handlePaste(event);
		document.addEventListener('paste', onPaste);

		return () => {
			subscription.unsubscribe();
			document.removeEventListener('paste', onPaste);
			// Final cleanup.
			this.thumbnails.forEach((t) => {
				if (t.src) URL.revokeObjectURL(t.src);
			});
		};
	};

	handleChange = async (event: Event) => {
		const target = event.target as HTMLInputElement;
		const files = target.files;

		if (files && files.length > 0) {
			for (const file of Array.from(files)) {
				await queuePhoto(this.treeId, file);
			}

			target.value = ''; // Reset to allow selecting the same file again.
		}
	};

	private handlePaste = async (event: ClipboardEvent) => {
		const items = event.clipboardData?.items;

		if (!items) return;

		for (const item of items) {
			if (item.kind === 'file') {
				const file = item.getAsFile();
				if (file && file.type.startsWith('image/')) {
					await queuePhoto(this.treeId, file, file.name || 'pasted-image');
				}
			}
		}
	};
}
