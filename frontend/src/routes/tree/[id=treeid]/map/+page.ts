import type { Load } from '@sveltejs/kit';
import { error } from '@sveltejs/kit';
import { apiClient } from '$lib/api';
import type { ISingleTree } from '$lib/types';

export const prerender = false;

export const load: Load = async ({
	params
}): Promise<{
	id: string;
	tree: ISingleTree;
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
