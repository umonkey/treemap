import { writable } from 'svelte/store';
import type { ITree } from '$lib/types';
import { mapBus } from '$lib/buses';
import { apiClient } from '$lib/api';

export const hook = (mount, destroy) => {
	const visible = writable<boolean>(false);
	const tree = writable<ITree | null>(null);
	const error = writable<string | null>(null);

	const handleClose = () => {
		visible.set(false);
		tree.set(null);
	};

	const handleSelect = (id: string) => {
		console.debug(`[MapPreview] Selected tree: ${id}`);

		apiClient.getTree(id).then((res) => {
			if (res.status === 200 && res.data) {
				tree.set(res.data);
				visible.set(true);
			} else if (res.error) {
				error.set(res.error.description);
			}
		});
	};

	mount(() => {
		mapBus.on('select', handleSelect);
	});

	destroy(() => {
		mapBus.off('select', handleSelect);
	});

	return { visible, error, tree, handleClose };
};
