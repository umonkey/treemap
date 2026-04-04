import { getTree } from '$lib/api/trees';
import { DEFAULT_TREE } from '$lib/constants';
import { routes } from '$lib/routes';
import type { ITree, ITreeFile } from '$lib/types';
import { writable } from 'svelte/store';

export const hooks = () => {
	const loading = writable<boolean>(true);
	const error = writable<string>('');
	const tree = writable<ITree>(DEFAULT_TREE);
	const images = writable<string[]>([]);

	const reload = (id: string) => {
		loading.set(true);

		getTree(id, true)
			.then((res) => {
				if (res.status === 200 && res.data) {
					tree.set(res.data);

					images.set(res.data.files.map((file: ITreeFile): string => routes.file(file.small_id)));

					error.set('');
				} else if (res.error) {
					error.set(res.error.description);
				}
			})
			.finally(() => {
				loading.set(false);
			});
	};

	return { loading, error, images, reload };
};
