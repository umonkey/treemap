import { getTreeFiles } from '$lib/api/trees';
import { uploadBus } from '$lib/buses/upload';
import { db } from '$lib/db';
import { locale } from '$lib/locale';
import { routes } from '$lib/routes';
import type { IGalleryItem, ITreeFile } from '$lib/types';
import { findSpeciesPhotos } from '$lib/utils/photos';
import { fileAttribution, formatDateShort } from '$lib/utils/strings';
import { liveQuery } from 'dexie';

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
	selected = $state<IGalleryItem | null>(null);
	private species = '';

	openViewer = (item: IGalleryItem) => {
		this.selected = item;
	};

	closeViewer = () => {
		this.selected = null;
	};

	reload = async (id: string, species: string) => {
		if (this.currentId === id && this.species === species) return;
		this.currentId = id;
		this.species = species;
		await this.fetch(id, false);
	};

	private fetch = async (id: string, silent: boolean) => {
		if (!silent) this.loading = true;

		try {
			const res = await getTreeFiles(id);
			if (res.status === 200 && res.data) {
				this.slides = this.formatSlides(res.data);
				this.error = null;
			} else if (res.error && !silent) {
				this.error = res.error.description;
			}
		} catch (err) {
			console.error('Failed to load gallery slides:', err);
			if (!silent) this.error = String(err);
		} finally {
			if (!silent) this.loading = false;
		}
	};

	watchUploads = (id: string) => {
		if (!id) return;

		let timer: ReturnType<typeof setTimeout> | undefined;

		const handler = (treeId: string) => {
			if (treeId !== id) return;

			if (timer) clearTimeout(timer);

			timer = setTimeout(() => {
				timer = undefined;
				void this.fetch(id, true);
			}, 2000);
		};

		uploadBus.on('success', handler);

		return () => {
			uploadBus.off('success', handler);
			if (timer) clearTimeout(timer);
		};
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

	// Real photos come first, followed by the species examples. Both are shaped as
	// gallery items so the component can render a single, uniform list.
	private formatSlides = (files: ITreeFile[]): IGalleryItem[] => {
		const photos = files.map((file: ITreeFile): IGalleryItem => {
			const label = this.label(file);

			return {
				id: file.id,
				small: routes.file(file.small_id),
				large: routes.file(file.source_id ?? file.large_id),
				label,
				description: fileAttribution(file) || label
			};
		});

		const examples = findSpeciesPhotos(this.species).map(
			(url: string, index: number): IGalleryItem => {
				return {
					id: `example-${index}`,
					small: url,
					large: url,
					label: locale.photoExample(),
					description: locale.photoExampleDescription()
				};
			}
		);

		return [...photos, ...examples];
	};

	private label = (file: ITreeFile): string => {
		if (!file.added_at) {
			return '';
		}

		return formatDateShort(file.added_at);
	};
}
