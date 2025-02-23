import type { Load } from '@sveltejs/kit';
import { apiClient } from '$lib/api';
import type { IStreetStats } from '$lib/types';
import { error } from '@sveltejs/kit';

export const load: Load = async (): Promise<{
	stats: IStreetStats[];
}> => {
	const { status, data } = await apiClient.getTopStreets();

	if (status === 200 && data) {
		return {
			stats: data
		};
	}

	error(404);
};
