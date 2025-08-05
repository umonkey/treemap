import { writable } from 'svelte/store';
import { type IHeatMap } from '$lib/types';
import { apiClient } from '$lib/api';

export const hooks = () => {
	const data = writable<IHeatMap[]>([]);
	const error = writable<string | undefined>(undefined);
	const loading = writable<boolean>(false);

	const reload = async () => {
		loading.set(true);

		try {
			const { status, data: d, error: e } = await apiClient.getHeatMap();

			if (status === 200 && d) {
				data.set(d);
			}

			if (e) {
				error.set(e.description);
			}
		} finally {
			loading.set(false);
		}
	};

	reload();

	return { data, error, loading };
};
