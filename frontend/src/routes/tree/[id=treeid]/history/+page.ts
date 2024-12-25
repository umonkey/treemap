import type { ITree } from '$lib/types';
import type { Load } from '@sveltejs/kit';
import { apiClient } from '$lib/api';
import { error } from '@sveltejs/kit';

type Response = {
	id: string;
	tree: ITree;
};

export const load: Load = async ({ params }): Promise<Response> => {
	const treeId = params.id;

	if (!treeId) {
		error(400, {
			message: 'Tree id not specified.'
		});
	}

	const res = await apiClient.getTree(treeId);

	if (res.status !== 200) {
		error(404, {
			message: 'Error fetching tree.'
		});
	}

	return {
		id: treeId,
		tree: res.data
	};
};
