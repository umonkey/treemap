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
	const res = await apiClient.getTopCircumference();

	if (res.status !== 200) {
		error(res.status);
	}

	addTrees(res.data.trees);
	addUsers(res.data.users);

	return {
		trees: res.data.trees,
		users: res.data.users
	};
};
