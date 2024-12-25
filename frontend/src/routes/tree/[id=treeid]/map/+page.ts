import type { Load } from '@sveltejs/kit';
import { error } from '@sveltejs/kit';
import { apiClient } from '$lib/api';

export const prerender = false;

export const load: Load = async ({ params }) => {
	const treeId = params.id;

	if (!treeId) {
		error(404);
	}

	const res = await apiClient.getTree(treeId);

	if (res.status !== 200) {
		error(404);
	}

	return {
		id: treeId,
		tree: res.data
	};
};
