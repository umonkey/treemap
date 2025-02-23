import type { Load } from '@sveltejs/kit';
import type { ITree } from '$lib/types';
import { error } from '@sveltejs/kit';
import { apiClient } from '$lib/api';

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

	const { status, data } = await apiClient.getTree(treeId);

	if (status === 200 && data) {
		return {
			id: treeId,
			tree: data
		};
	}

	error(status);
};
