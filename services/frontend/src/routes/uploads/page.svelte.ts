import { type IUpload, db } from '$lib/db';
import { processUploadQueue, restartUploadQueue } from '$lib/upload';
import { liveQuery } from 'dexie';

class PageState {
	uploads = $state<IUpload[]>([]);

	onMount = () => {
		const query = liveQuery(() => db.uploads.orderBy('created_at').reverse().toArray());

		const subscription = query.subscribe({
			next: (data) => {
				this.uploads = data;
			},
			error: (err) => console.error('Failed to load uploads:', err)
		});

		return () => subscription.unsubscribe();
	};

	processQueue = () => {
		processUploadQueue();
	};

	restartQueue = () => {
		restartUploadQueue();
	};
}

export const pageState = new PageState();
