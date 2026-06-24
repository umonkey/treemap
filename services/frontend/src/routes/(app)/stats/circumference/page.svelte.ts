import { getTopCircumference } from '$lib/api/stats';
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

			const { status, data: list, error: err } = await getTopCircumference();

			if (status === 200 && list) {
				addTrees(list.trees);
				addUsers(list.users);

				this.data = list.trees;
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
