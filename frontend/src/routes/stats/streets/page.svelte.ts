import { getTopStreets } from '$lib/api/stats';
import type { IError, IStreetStats } from '$lib/types';

class PageState {
	loading = $state<boolean>(true);
	data = $state<IStreetStats[]>([]);
	error = $state<IError | undefined>(undefined);

	reload = async () => {
		try {
			this.loading = true;

			const { status, data: stats, error: err } = await getTopStreets();

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

	reorder = (field: string): void => {
		if (field === 'address') {
			this.data = [...this.data].sort((a, b) => a.address.localeCompare(b.address));
		} else {
			this.data = [...this.data].sort((a, b) => b.count - a.count);
		}
	};
}

export const pageState = new PageState();
