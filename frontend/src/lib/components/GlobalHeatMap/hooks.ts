import { getHeatMap } from '$lib/api/stats';
import { type IHeatMap } from '$lib/types';
import { writable } from 'svelte/store';

export const hooks = () => {
	const data = writable<IHeatMap[]>([]);
	const error = writable<string | undefined>(undefined);
	const loading = writable<boolean>(false);

	const reload = async () => {
		loading.set(true);

		try {
			const { status, data: d, error: e } = await getHeatMap();

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
