import { getTreeFiles } from '$lib/api/trees';
import { routes } from '$lib/routes';
import { getUser } from '$lib/stores/userStore';
import type { IGalleryItem, ITreeFile } from '$lib/types';
import { formatDate } from '$lib/utils/strings';
import { get } from 'svelte/store';

class GalleryState {
	loading = $state<boolean>(true);
	error = $state<string | null>(null);
	slides = $state<IGalleryItem[]>([]);
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

	private formatSlides(files: ITreeFile[]): IGalleryItem[] {
		return files.map((file: ITreeFile): IGalleryItem => {
			return {
				id: file.id,
				small: routes.file(file.small_id),
				large: routes.file(file.source_id ?? file.large_id),
				label: this.added_at(file)
			};
		});
	}

	private added_at(file: ITreeFile): string {
		if (!file.added_at || !file.added_by) {
			return '';
		}

		const user = get(getUser)(file.added_by);

		if (user === undefined) {
			return '';
		}

		const date = formatDate(file.added_at);
		return `${date} by ${user.name}`;
	}
}

export const galleryState = new GalleryState();
