import { apiClient } from "$lib/api";
import type { IError, IStateStats } from "$lib/types";
import { writable } from "svelte/store";

export const loadStateStats = () => {
	const loading = writable<boolean>(true);
	const data = writable<IStateStats[]>([]);
	const error = writable<IError | undefined>(undefined);

	const reload = async () => {
		try {
			loading.set(true);

			const {
				status,
				data: stats,
				error: err,
			} = await apiClient.getStateStats();

			if (status === 200 && stats) {
				data.set(stats);
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
