import { changeTreeThumbnail, getTree } from '$lib/api/trees';
import { deleteFile } from '$lib/api/uploads';
import { DEFAULT_TREE } from '$lib/constants';
import { showError } from '$lib/errors';
import type { ITree, ITreeFile } from '$lib/types';
import { writable } from 'svelte/store';
import { get } from 'svelte/store';

export const hooks = () => {
	const loading = writable<boolean>(false);
	const tree = writable<ITree>(DEFAULT_TREE);
	const error = writable<string | null>(null);

	const reload = (id: string) => {
		loading.set(true);

		getTree(id, true)
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
		const res = await changeTreeThumbnail(get(tree).id, file.id);

		if (res.status >= 400) {
			showError(`Error ${res.status} changing thumbnail.`);
		}
	};

	const handleDelete = async (id: string) => {
		const res = await deleteFile(id);

		if (res.status >= 400) {
			showError(`Error ${res.status} deleting file.`);
		}
	};

	return { loading, tree, error, reload, handleMakeThumbnail, handleDelete };
};
