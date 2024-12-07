import type { Load } from '@sveltejs/kit';
import { error } from '@sveltejs/kit';
import { apiClient } from '$lib/api';

export const load: Load = async ({ params }) => {
	const res = await apiClient.getTree(params.id);

	if (res.status !== 200) {
		error(404);
	}

	return {
		id: params.id,
		tree: res.data
	};
};
