import { getStateStats } from '$lib/api/stats';
import type { IError, IStateStats } from '$lib/types';

class PageState {
	loading = $state<boolean>(true);
	data = $state<IStateStats[]>([]);
	error = $state<IError | undefined>(undefined);

	reload = async () => {
		try {
			this.loading = true;

			const { status, data: stats, error: err } = await getStateStats();

			if (status === 200 && stats) {
				this.data = stats;
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
