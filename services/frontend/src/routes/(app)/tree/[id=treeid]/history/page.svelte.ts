import { getTree, getTreeHistory } from '$lib/api/trees';
import { addUsers } from '$lib/stores/userStore';
import type { IChange, IError, ISingleTree } from '$lib/types';

class PageState {
	loading = $state<boolean>(true);
	error = $state<IError | undefined>(undefined);
	tree = $state<ISingleTree | undefined>(undefined);
	changes = $state<IChange[]>([]);

	reload = async (id: string) => {
		try {
			this.loading = true;
			this.error = undefined;
			this.tree = undefined;
			this.changes = [];

			const req1 = await getTree(id);

			if (req1.status === 200 && req1.data) {
				const req2 = await getTreeHistory(id);

				if (req2.status === 200 && req2.data) {
					addUsers(req2.data.users);
					this.tree = req1.data;
					this.changes = req2.data.props;
				} else {
					this.error = req2.error;
				}
			} else {
				this.error = req1.error;
				return;
			}
		} finally {
			this.loading = false;
		}
	};
}

export const pageState = new PageState();
