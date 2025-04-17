import { apiClient } from "$lib/api";
import { addTrees } from "$lib/stores/treeStore";
import { addUsers } from "$lib/stores/userStore";
import type { IError, ITree } from "$lib/types";
import { writable } from "svelte/store";

export const loadSpeciesMismatch = () => {
	const loading = writable<boolean>(true);
	const data = writable<ITree[]>([]);
	const error = writable<IError | undefined>(undefined);

	const reload = async () => {
		try {
			loading.set(true);

			const {
				status,
				data: stats,
				error: err,
			} = await apiClient.getSpeciesMismatch();

			if (status === 200 && stats) {
				addTrees(stats.trees);
				addUsers(stats.users);

				data.set(stats.trees);
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
