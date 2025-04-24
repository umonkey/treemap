// This hook loads data on the tree and its change history.

import { apiClient } from '$lib/api';
import { addUsers } from '$lib/stores/userStore';
import type { IChange, IError, ISingleTree } from '$lib/types';
import { writable } from 'svelte/store';

export const loadTreeHistory = () => {
	const loading = writable<boolean>(true);
	const error = writable<IError | undefined>(undefined);
	const tree = writable<ISingleTree | undefined>(undefined);
	const changes = writable<IChange[]>([]);

	const reload = async (id: string) => {
		try {
			loading.set(true);
			error.set(undefined);
			tree.set(undefined);
			changes.set([]);

			const req1 = await apiClient.getTree(id);

			if (req1.status === 200 && req1.data) {
				const req2 = await apiClient.getTreeHistory(id);

				if (req2.status === 200 && req2.data) {
					addUsers(req2.data.users);
					tree.set(req1.data);
					changes.set(req2.data.props);
				} else {
					error.set(req2.error);
				}
			} else {
				error.set(req1.error);
				return;
			}
		} finally {
			loading.set(false);
		}
	};

	return { loading, tree, changes, error, reload };
};
