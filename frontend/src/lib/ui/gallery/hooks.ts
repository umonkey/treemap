import { writable } from 'svelte/store';
import type { IGalleryItem, ITreeFile } from '$lib/types';
import { apiClient } from '$lib/api';
import { get } from 'svelte/store';
import { getUser } from '$lib/stores/userStore';
import { routes } from '$lib/routes';
import { formatDate } from '$lib/utils/strings';

const added_at = (file: ITreeFile) => {
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

const formatSlides = (files: ITreeFile[]): IGalleryItem[] => {
	const items = files.map((file: ITreeFile): IGalleryItem => {
		return {
			small: routes.file(file.small_id),
			large: routes.file(file.large_id),
			label: added_at(file)
		};
	});

	return items;
};

export const hooks = () => {
	const loading = writable<boolean>(true);
	const error = writable<string | null>(null);
	const slides = writable<IGalleryItem[]>([]);

	const reload = (id: string) => {
		apiClient
			.getTreeFiles(id)
			.then((res) => {
				if (res.status === 200 && res.data) {
					slides.set(formatSlides(res.data));
					error.set(null);
				} else if (res.error) {
					error.set(res.error.description);
				}
			})
			.finally(() => {
				loading.set(false);
			});
	};

	return { loading, error, slides, reload };
};
