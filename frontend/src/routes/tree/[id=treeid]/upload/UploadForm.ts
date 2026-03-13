import { writable } from 'svelte/store';
import type { ITree, ITreeFile } from '$lib/types';
import { apiClient } from '$lib/api';
import { showError } from '$lib/errors';
import { get } from 'svelte/store';
import { DEFAULT_TREE } from '$lib/constants';

export const hooks = () => {
	const loading = writable<boolean>(false);
	const tree = writable<ITree>(DEFAULT_TREE);
	const error = writable<string | null>(null);

	const reload = (id: string) => {
		loading.set(true);

		apiClient
			.getTree(id, true)
			.then((res) => {
				if (res.status === 200 && res.data) {
					tree.set(res.data);
					error.set(null);
				} else if (res.error) {
					error.set(res.error.description);
				}
			})
			.finally(() => {
				loading.set(false);
			});
	};

	const handleMakeThumbnail = async (file: ITreeFile) => {
		const res = await apiClient.changeTreeThumbnail(get(tree).id, file.id);

		if (res.status >= 400) {
			showError('Error changing thumbnail.');
		}
	};

	const handleDelete = async (id: string) => {
		const res = await apiClient.deleteFile(id);

		if (res.status >= 400) {
			showError('Error deleting file.');
		}
	};

	return { loading, tree, error, reload, handleMakeThumbnail, handleDelete };
};
