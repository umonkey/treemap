/**
 * Load trees ordered by trunk circumference
 */

import { apiClient } from '$lib/api';
import { addTrees } from '$lib/stores/treeStore';
import { addUsers } from '$lib/stores/userStore';
import type { IError, ITree } from '$lib/types';
import { writable } from 'svelte/store';

export const loadTreesByCircumference = () => {
	const loading = writable<boolean>(true);
	const data = writable<ITree[]>([]);
	const error = writable<IError | undefined>(undefined);

	const reload = async () => {
		try {
			loading.set(true);

			const { status, data: list, error: err } = await apiClient.getTopCircumference();

			if (status === 200 && list) {
				addTrees(list.trees);
				addUsers(list.users);

				data.set(list.trees);
				error.set(undefined);
			} else {
				data.set([]);
				error.set(err);
			}
		} finally {
			loading.set(false);
		}
	};

	return { loading, error, data, reload };
};
