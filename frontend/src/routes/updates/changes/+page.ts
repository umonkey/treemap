import { apiClient } from "$lib/api";
import { addTrees } from "$lib/stores/treeStore";
import { addUsers } from "$lib/stores/userStore";
import type { ITree, IUser } from "$lib/types";
import type { Load } from "@sveltejs/kit";
import { error } from "@sveltejs/kit";

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
			users: data.users,
		};
	}

	error(status);
};
