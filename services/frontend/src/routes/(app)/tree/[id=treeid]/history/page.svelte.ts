import { getTreeHistory } from '$lib/api/trees';
import { addUsers } from '$lib/stores/userStore';
import type { IChange, IError } from '$lib/types';

class PageState {
	loading = $state<boolean>(true);
	error = $state<IError | undefined>(undefined);
	changes = $state<IChange[]>([]);

	reload = async (id: string) => {
		try {
			this.loading = true;
			this.error = undefined;
			this.changes = [];

			const res = await getTreeHistory(id);

			if (res.status === 200 && res.data) {
				addUsers(res.data.users);
				this.changes = res.data.props;
			} else {
				this.error = res.error;
			}
		} finally {
			this.loading = false;
		}
	};
}

export const pageState = new PageState();
