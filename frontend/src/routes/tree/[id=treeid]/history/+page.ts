import { apiClient } from '$lib/api';
import type { ITree } from '$lib/types';
import type { Load } from '@sveltejs/kit';
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

	const { status, data } = await apiClient.getTree(treeId);

	if (status === 200 && data) {
		return {
			id: treeId,
			tree: data
		};
	}

	error(status);
};
