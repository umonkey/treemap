import { db } from '$lib/db';
import { addPhotoToUploadQueue } from '$lib/upload';
import { liveQuery } from 'dexie';

export type Thumbnail = {
	file: Blob | File;
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
				this.thumbnails = data.map((item) => ({
					file: item.thumbnail || item.image,
					busy: item.status === 'uploading',
					error: item.status === 'failed'
				}));
				this.onChange(data.length);
			},
			error: (err) => console.error('Failed to load thumbnails:', err)
		});

		const onPaste = (event: ClipboardEvent) => this.handlePaste(event);
		document.addEventListener('paste', onPaste);

		return () => {
			subscription.unsubscribe();
			document.removeEventListener('paste', onPaste);
		};
	}

	private appendFile = async (file: File) => {
		await addPhotoToUploadQueue(this.treeId, file);
	};

	handleChange = async (event: Event) => {
		const files = (event.target as HTMLInputElement).files;

		if (files && files.length > 0) {
			for (let i = 0; i < files.length; i++) {
				const file = files[i];
				await this.appendFile(file);
			}
		}
	};

	private handlePaste = async (event: ClipboardEvent) => {
		const items = event.clipboardData?.items;

		if (!items) return;

		for (const item of items) {
			if (item.kind === 'file') {
				const file = item.getAsFile();
				if (file && file.type.startsWith('image/')) {
					await this.appendFile(file);
				}
			}
		}
	};
}

export const componentState = new ComponentState();
