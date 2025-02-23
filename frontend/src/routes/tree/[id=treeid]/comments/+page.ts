import type { Load } from '@sveltejs/kit';
import type { ITree, IComment } from '$lib/types';
import { error } from '@sveltejs/kit';
import { apiClient } from '$lib/api';
import { addUsers } from '$lib/stores/userStore';

const loadTree = async (id: string): Promise<ITree> => {
	const res = await apiClient.getTree(id);

	if (res.status !== 200) {
		error(404);
	}

	addUsers(res.data.users);

	return res.data;
};

const loadComments = async (id: string): Promise<IComment[]> => {
	const res = await apiClient.getTreeComments(id);

	if (res.status !== 200) {
		error(404);
	}

	addUsers(res.data.users);

	return res.data.comments;
};

export const load: Load = async ({
	params
}): Promise<{
	id: string;
	tree: ITree;
	comments: IComment[];
}> => {
	const treeId = params.id;

	if (!treeId) {
		error(400);
	}

	return {
		id: treeId,
		tree: await loadTree(treeId),
		comments: await loadComments(treeId)
	};
};
