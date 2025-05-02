import { writable } from 'svelte/store';
import type { ITreeFile } from '$lib/types';
import { apiClient } from '$lib/api';
import { get } from 'svelte/store';
import { getUser } from '$lib/stores/userStore';
import { routes } from '$lib/routes';
import { formatDate } from '$lib/utils/strings';

export const hooks = () => {
	const loading = writable<boolean>(true);
	const error = writable<string | null>(null);
	const files = writable<ITreeFile[]>([]);

	const reload = (id: string) => {
		apiClient
			.getTreeFiles(id)
			.then((res) => {
				if (res.status === 200 && res.data) {
					files.set(res.data);
					error.set(null);
				} else if (res.error) {
					error.set(res.error.description);
				}
			})
			.finally(() => {
				loading.set(false);
			});
	};

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

	const handleExpand = (file: ITreeFile) => {
		const url = routes.file(file.id);
		window.open(url, '_blank');
	};

	return { loading, error, files, reload, added_at, handleExpand };
};
