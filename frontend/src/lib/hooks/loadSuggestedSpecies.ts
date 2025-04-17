import { apiClient } from "$lib/api";
import type { IError } from "$lib/types";
import { writable } from "svelte/store";

export const loadSuggestedSpecies = () => {
	const loading = writable<boolean>(true);
	const data = writable<string[]>([]);
	const error = writable<IError | undefined>(undefined);

	const reload = async () => {
		try {
			loading.set(true);

			const {
				status,
				data: stats,
				error: err,
			} = await apiClient.suggestSpecies();

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
