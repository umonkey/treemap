import { apiClient } from '$lib/api';
import type { IError, ISpeciesStats } from '$lib/types';
import { writable } from 'svelte/store';

export const loadSpeciesStats = () => {
	const loading = writable<boolean>(true);
	const data = writable<ISpeciesStats[]>([]);
	const error = writable<IError | undefined>(undefined);

	const reload = async () => {
		try {
			loading.set(true);

			const { status, data: stats, error: err } = await apiClient.getSpeciesStats();

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
			if (field === 'count') {
				return items.sort((a, b) => b.count - a.count);
			}

			return items.sort((a, b) => a.name.localeCompare(b.name));
		});
	};

	return { loading, error, data, reload, reorder };
};
