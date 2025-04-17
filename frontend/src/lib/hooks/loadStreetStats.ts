import { apiClient } from "$lib/api";
import type { IError, IStreetStats } from "$lib/types";
import { writable } from "svelte/store";

export const loadStreetStats = () => {
	const loading = writable<boolean>(true);
	const data = writable<IStreetStats[]>([]);
	const error = writable<IError | undefined>(undefined);

	const reload = async () => {
		try {
			loading.set(true);

			const {
				status,
				data: stats,
				error: err,
			} = await apiClient.getTopStreets();

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

	const reorder = (field: string): void => {
		data.update((items) => {
			if (field === "address") {
				return items.sort((a, b) => a.address.localeCompare(b.address));
			}

			return items.sort((a, b) => b.count - a.count);
		});
	};

	return { loading, error, data, reload, reorder };
};
