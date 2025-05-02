import { writable } from 'svelte/store';
import type { ITree, ITreeFile } from '$lib/types';
import { apiClient } from '$lib/api';
import { toast } from '@zerodevx/svelte-toast';
import { get } from 'svelte/store';
import { DEFAULT_TREE } from '$lib/constants';

export const hooks = () => {
	const loading = writable<boolean>(false);
	const tree = writable<ITree>(DEFAULT_TREE);
	const error = writable<string | null>(null);
	const thumbnail = writable<string | null>(null);

	const reload = (id: string) => {
		loading.set(true);

		apiClient
			.getTree(id, false)
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

		if (res.status >= 200 && res.status < 300) {
			thumbnail.set(file.small_id);
			toast.push('Thumbnail changed.');
		} else {
			toast.push('Error changing thumbnail.');
		}
	};

	const handleDelete = async (id: string) => {
		const res = await apiClient.deleteFile(id);

		if (res.status >= 200 && res.status < 300) {
			toast.push('File deleted.');
		} else {
			toast.push('Error deleting file.');
		}
	};

	return { loading, tree, thumbnail, error, reload, handleMakeThumbnail, handleDelete };
};
