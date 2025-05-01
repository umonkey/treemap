import { writable } from 'svelte/store';
import { apiClient } from '$lib/api';
import { DEFAULT_TREE } from '$lib/constants';
import type { ITree } from '$lib/types';

export const hooks = () => {
	const loading = writable<boolean>(true);
	const error = writable<string>('');
	const tree = writable<ITree>(DEFAULT_TREE);

	const reload = (id: string) => {
		loading.set(true);

		apiClient
			.getTree(id, true)
			.then((res) => {
				if (res.status === 200 && res.data) {
					tree.set(res.data);
					error.set('');
				} else if (res.error) {
					error.set(res.error.description);
				}
			})
			.finally(() => {
				loading.set(false);
			});
	};

	return { loading, error, tree, reload };
};
