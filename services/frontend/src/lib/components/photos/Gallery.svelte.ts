import { getTreeFiles } from '$lib/api/trees';
import { db } from '$lib/db';
import { routes } from '$lib/routes';
import { getUser } from '$lib/stores/userStore';
import type { IGalleryItem, ITreeFile } from '$lib/types';
import { formatDate } from '$lib/utils/strings';
import { liveQuery } from 'dexie';
import { get } from 'svelte/store';

export type QueuedPhoto = {
	id: number;
	src: string;
	busy: boolean;
};

export class Gallery {
	loading = $state<boolean>(true);
	error = $state<string | null>(null);
	slides = $state<IGalleryItem[]>([]);
	queue = $state<QueuedPhoto[]>([]);
	currentId = $state<string | null>(null);

	reload = async (id: string) => {
		if (this.currentId === id) return;
		this.currentId = id;
		this.loading = true;

		try {
			const res = await getTreeFiles(id);
			if (res.status === 200 && res.data) {
				this.slides = this.formatSlides(res.data);
				this.error = null;
			} else if (res.error) {
				this.error = res.error.description;
			}
		} catch (err) {
			console.error('Failed to load gallery slides:', err);
			this.error = String(err);
		} finally {
			this.loading = false;
		}
	};

	watchQueue = (id: string) => {
		if (!id) return;

		const query = liveQuery(() => db.uploads.where('tree_id').equals(id.toString()).toArray());

		const subscription = query.subscribe({
			next: (data) => {
				// Cleanup existing URLs to avoid memory leaks.
				this.queue.forEach((q) => URL.revokeObjectURL(q.src));

				this.queue = data
					.map((item) => ({
						id: item.id!,
						src: URL.createObjectURL(item.thumbnail || item.image),
						busy: item.status === 'uploading',
						created_at: item.created_at
					}))
					.sort((a, b) => b.created_at - a.created_at || b.id - a.id)
					.map((item): QueuedPhoto => ({ id: item.id, src: item.src, busy: item.busy }));
			},
			error: (err) => console.error('Failed to load upload queue:', err)
		});

		return () => {
			subscription.unsubscribe();
			this.queue.forEach((q) => URL.revokeObjectURL(q.src));
		};
	};

	private formatSlides = (files: ITreeFile[]): IGalleryItem[] => {
		return files.map((file: ITreeFile): IGalleryItem => {
			return {
				id: file.id,
				small: routes.file(file.small_id),
				large: routes.file(file.source_id ?? file.large_id),
				label: this.added_at(file)
			};
		});
	};

	private added_at = (file: ITreeFile): string => {
		if (!file.added_at || !file.added_by) {
			return '';
		}

		const user = get(getUser)(file.added_by);

		if (user === undefined) {
			return '';
		}

		const date = formatDate(file.added_at);
		return `${date} by ${user.name}`;
	};
}
