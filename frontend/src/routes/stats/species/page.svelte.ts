import { getSpeciesStats } from '$lib/api/stats';
import type { IError, ISpeciesStats } from '$lib/types';

class PageState {
	loading = $state<boolean>(true);
	data = $state<ISpeciesStats[]>([]);
	error = $state<IError | undefined>(undefined);

	reload = async () => {
		try {
			this.loading = true;

			const { status, data: stats, error: err } = await getSpeciesStats();

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
		if (field === 'count') {
			this.data = [...this.data].sort((a, b) => b.count - a.count);
		} else {
			this.data = [...this.data].sort((a, b) => a.name.localeCompare(b.name));
		}
	};
}

export const pageState = new PageState();
