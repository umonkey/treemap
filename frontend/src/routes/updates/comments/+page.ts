import type { Load } from '@sveltejs/kit';
import { apiClient } from '$lib/api';
import { error } from '@sveltejs/kit';
import { addUsers } from '$lib/stores/userStore';
import { addTrees } from '$lib/stores/treeStore';
import type { IComment } from '$lib/types';

export const load: Load = async (): Promise<{
	comments: IComment[];
}> => {
	const res = await apiClient.getRecentComments();

	if (res.status !== 200) {
		error(res.status);
	}

	addUsers(res.data.users);
	addTrees(res.data.trees);

	return {
		comments: res.data.comments,
	};
};
