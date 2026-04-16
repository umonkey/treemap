import { getSpeciesMismatch } from '$lib/api/stats';
import { addTrees } from '$lib/stores/treeStore';
import { addUsers } from '$lib/stores/userStore';
import type { IError, ITree } from '$lib/types';

class PageState {
	loading = $state<boolean>(true);
	data = $state<ITree[]>([]);
	error = $state<IError | undefined>(undefined);

	reload = async () => {
		try {
			this.loading = true;

			const { status, data: stats, error: err } = await getSpeciesMismatch();

			if (status === 200 && stats) {
				addTrees(stats.trees);
				addUsers(stats.users);

				this.data = stats.trees;
				this.error = undefined;
			} else {
				this.data = [];
				this.error = err;
			}
		} finally {
			this.loading = false;
		}
	};
}

export const pageState = new PageState();
