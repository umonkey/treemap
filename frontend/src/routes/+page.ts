import { apiClient } from '$lib/api';
import type { Load } from '@sveltejs/kit';

export const load: Load = async () => {
	const res = await apiClient.getStats();

	return {
		totalCount: res.data?.count ?? 0
	};
};
