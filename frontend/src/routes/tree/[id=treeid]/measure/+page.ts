import type { Load } from '@sveltejs/kit';
import { error } from '@sveltejs/kit';
import { apiClient } from '$lib/api';
import { addTrees } from '$lib/stores/treeStore';

export const load: Load = async ({
	params
}): Promise<{
	treeId: string;
}> => {
	const treeId = params.id;

	if (!treeId) {
		error(404);
	}

	const res = await apiClient.getTree(treeId);

	if (res.status !== 200) {
		error(404);
	}

	addTrees([res.data]);

	return {
		treeId
	};
};
