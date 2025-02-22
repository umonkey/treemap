import type { ISpeciesStats } from '$lib/types';
import type { Load } from '@sveltejs/kit';
import { apiClient } from '$lib/api';
import { error } from '@sveltejs/kit';

export const load: Load = async (): Promise<{
	stats: ISpeciesStats[];
}> => {
	const res = await apiClient.getSpeciesStats();

	if (res.status !== 200) {
		error(res.status);
	}

	return {
		stats: res.data
	};
};
