import type { Load } from '@sveltejs/kit';
import { error } from '@sveltejs/kit';
import { apiClient } from '$lib/api';
import type { ITree } from '$lib/types';
import { addTrees } from '$lib/stores/treeStore';

export const load: Load = async ({
	params
}): Promise<{
	id: string;
	tree: ITree;
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
