import { getTreeHistory } from '$lib/api/trees';
import { addUsers } from '$lib/stores/userStore';
import type { IChange, IError } from '$lib/types';

export class PageState {
	loading = $state<boolean>(true);
	error = $state<IError | undefined>(undefined);
	changes = $state<IChange[]>([]);

	reload = async (id: string) => {
		try {
			this.loading = true;
			this.error = undefined;

			const res = await getTreeHistory(id);

			if (res.status === 200 && res.data) {
				addUsers(res.data.users);
				this.changes = res.data.props;
			} else {
				this.changes = [];
				this.error = res.error;
			}
		} finally {
			this.loading = false;
		}
	};
}
