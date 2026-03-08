import { apiClient } from '$lib/api';
import { type IHeatMap } from '$lib/types';
import { writable } from 'svelte/store';

export const hooks = (user_id: string) => {
	const data = writable<IHeatMap[]>([]);
	const error = writable<string | undefined>(undefined);
	const loading = writable<boolean>(false);

	const reload = async () => {
		loading.set(true);

		try {
			const { status, data: d, error: e } = await apiClient.getUserHeatMap(user_id);

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
