import type { Load } from '@sveltejs/kit';
import { apiClient } from '$lib/api';
import { error } from '@sveltejs/kit';
import { addUsers } from '$lib/stores/userStore';
import { addTrees } from '$lib/stores/treeStore';
import type { ITree, IUser } from '$lib/types';

export const load: Load = async (): Promise<{
	trees: ITree[];
	users: IUser[];
}> => {
	const { status, data } = await apiClient.getUpdatedTrees();

	if (status === 200 && data) {
		addTrees(data.trees);
		addUsers(data.users);

		return {
			trees: data.trees,
			users: data.users
		};
	}

	error(status);
};
