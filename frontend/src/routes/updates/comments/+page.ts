import type { Load } from '@sveltejs/kit';
import { apiClient } from '$lib/api';
import { error } from '@sveltejs/kit';
import { addUsers } from '$lib/stores/userStore';
import { addTrees } from '$lib/stores/treeStore';

export const load: Load = async (): Promise<{
	comments: ICommentList;
}> => {
	const res = await apiClient.getRecentComments();

	if (res.status !== 200) {
		error('Failed to load comments');
	}

	addUsers(res.data.users);
	addTrees(res.data.trees);

	return {
		comments: res.data.comments,
		users: res.data.users,
		trees: res.data.trees
	};
};
