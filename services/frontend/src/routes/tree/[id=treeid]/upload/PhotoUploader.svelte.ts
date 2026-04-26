import { db } from '$lib/db';
import { showError } from '$lib/errors';
import { addPhotoToUploadQueue } from '$lib/upload';
import { liveQuery } from 'dexie';

export type Thumbnail = {
	file: Blob | File;
	src: string;
	busy: boolean;
	error: boolean;
};

class ComponentState {
	thumbnails = $state<Thumbnail[]>([]);
	private treeId = '';
	private onChange: (files: number) => void = () => {};

	init(treeId: string, onChange: (files: number) => void) {
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
	}

	private appendFile = async (file: File) => {
		await addPhotoToUploadQueue(this.treeId, file);
	};

	handleChange = async (event: Event) => {
		const target = event.target as HTMLInputElement;
		const files = target.files;

		if (files && files.length > 0) {
			const fileList = Array.from(files);

			for (const file of fileList) {
				try {
					// On some mobile devices, the File object might be invalidated if the input is cleared
					// or if we wait too long. Slicing it into a new Blob helps "solidify" the data.
					const blob = file.slice(0, file.size, file.type);
					const solidifiedFile = new File([blob], file.name, { type: file.type });

					await this.appendFile(solidifiedFile);
				} catch (e) {
					console.error('Failed to append file:', e);
					showError(
						`Failed to add photo "${file.name}": ${e instanceof Error ? e.message : 'Unknown error'}`
					);
				}
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
					const blob = file.slice(0, file.size, file.type);
					const solidifiedFile = new File([blob], file.name || 'pasted-image', { type: file.type });
					await this.appendFile(solidifiedFile);
				}
			}
		}
	};
}

export const componentState = new ComponentState();
