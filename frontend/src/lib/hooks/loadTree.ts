import type { IError, ISingleTree } from '$lib/types';
import { apiClient } from '$lib/api';
import { writable } from 'svelte/store';

export const loadTree = () => {
	const loading = writable<boolean>(true);
	const data = writable<ISingleTree | undefined>(undefined);
	const error = writable<IError | undefined>(undefined);

	const reload = async (id: string) => {
		try {
			loading.set(true);

			const { status, data: tree, error: err } = await apiClient.getTree(id);

			if (status === 200 && tree) {
				data.set(tree);
				error.set(undefined);
			} else {
				data.set(undefined);
				error.set(err);
			}
		} finally {
			loading.set(false);
		}
	};

	return { loading, data, error, reload };
};
