import { apiClient } from '$lib/api';
import { addTrees } from '$lib/stores/treeStore';
import type { Load } from '@sveltejs/kit';
import { error } from '@sveltejs/kit';

export const load: Load = async ({
	params
}): Promise<{
	treeId: string;
}> => {
	const treeId = params.id;

	if (!treeId) {
		error(404);
	}

	const { status, data } = await apiClient.getTree(treeId);

	if (status === 200 && data) {
		addTrees([data]);

		return {
			treeId
		};
	}

	error(status);
};
