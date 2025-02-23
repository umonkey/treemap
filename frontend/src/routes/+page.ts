import type { Load } from '@sveltejs/kit';
import { apiClient } from '$lib/api';

export const load: Load = async () => {
	const res = await apiClient.getStats();

	return {
		totalCount: res.data?.count ?? 0
	};
};
