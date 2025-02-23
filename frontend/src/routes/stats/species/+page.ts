import type { ISpeciesStats } from '$lib/types';
import type { Load } from '@sveltejs/kit';
import { apiClient } from '$lib/api';
import { error } from '@sveltejs/kit';

export const load: Load = async (): Promise<{
	stats: ISpeciesStats[];
}> => {
	const { status, data } = await apiClient.getSpeciesStats();

	console.debug('RES', status, data);

	if (status === 200 && data) {
		return {
			stats: data
		};
	}

	error(400);
};
