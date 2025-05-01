import { writable } from 'svelte/store';
import type { ITree } from '$lib/types';
import { apiClient } from '$lib/api';
import { routes, goto } from '$lib/routes';

export const hook = () => {
	const visible = writable<boolean>(false);
	const tree = writable<ITree | null>(null);
	const error = writable<string | null>(null);

	const handleClose = () => {
		goto(routes.map());
	};

	const reload = (id: string | null) => {
		console.debug(`[MapPreview] Selected tree: ${id}`);

		if (id === null) {
			visible.set(false);
			tree.set(null);
			return;
		}

		apiClient.getTree(id).then((res) => {
			if (res.status === 200 && res.data) {
				tree.set(res.data);
				visible.set(true);
			} else if (res.error) {
				error.set(res.error.description);
			}
		});
	};

	return { visible, error, tree, reload, handleClose };
};
