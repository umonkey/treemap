import { apiClient } from '$lib/api';
import { addUsers } from '$lib/stores/userStore';
import type { IComment, ITree } from '$lib/types';
import type { Load } from '@sveltejs/kit';
import { error } from '@sveltejs/kit';

const loadTree = async (id: string): Promise<ITree> => {
	const { status, data } = await apiClient.getTree(id);

	if (status === 200 && data) {
		addUsers(data.users);
		return data;
	}

	error(status);
};

const loadComments = async (id: string): Promise<IComment[]> => {
	const { status, data } = await apiClient.getTreeComments(id);

	if (status === 200 && data) {
		addUsers(data.users);

		return data.comments;
	}

	error(status);
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
