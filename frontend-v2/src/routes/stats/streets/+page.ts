import type { Load } from '@sveltejs/kit';
import { apiClient } from '$lib/api';

export const load: Load = async (): Promise<{
	stats: IStreetStats[];
}> => {
	const res = await apiClient.getTopStreets();

	if (res.status !== 200) {
		error(404);
	}

	return {
		stats: res.data
	};
};
