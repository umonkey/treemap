import { getHeatMap } from '$lib/api/stats';
import { type IHeatMap } from '$lib/types';

class GlobalHeatMapState {
	data = $state<IHeatMap[]>([]);
	error = $state<string | undefined>(undefined);
	loading = $state<boolean>(false);
	initialized = false;

	public init = async () => {
		if (this.initialized) return;
		this.initialized = true;
		await this.reload();
	};

	public reload = async () => {
		this.loading = true;

		try {
			const { status, data: d, error: e } = await getHeatMap();

			if (status === 200 && d) {
				this.data = d;
			}

			if (e) {
				this.error = e.description;
			}
		} finally {
			this.loading = false;
		}
	};
}

export const globalHeatMapState = new GlobalHeatMapState();
