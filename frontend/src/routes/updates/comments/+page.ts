import { apiClient } from '$lib/api';
import { addTrees } from '$lib/stores/treeStore';
import { addUsers } from '$lib/stores/userStore';
import type { IComment } from '$lib/types';
import type { Load } from '@sveltejs/kit';
import { error } from '@sveltejs/kit';

export const load: Load = async (): Promise<{
	comments: IComment[];
}> => {
	const { status, data } = await apiClient.getRecentComments();

	if (status === 200 && data) {
		addUsers(data.users);
		addTrees(data.trees);

		return {
			comments: data.comments
		};
	}

	error(status);
};
