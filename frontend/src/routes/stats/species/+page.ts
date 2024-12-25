import type { Load } from '@sveltejs/kit';
import { apiClient } from '$lib/api';

export const load: Load = async (): Promise<{
	stats: ISpeciesStats[];
}> => {
	const res = await apiClient.getSpeciesStats();

	if (res.status !== 200) {
		error(404);
	}

	return {
		stats: res.data
	};
};
