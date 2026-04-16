import { getUserHeatMap } from '$lib/api/users';
import { type IHeatMap } from '$lib/types';

class UserHeatMapState {
	data = $state<IHeatMap[]>([]);
	error = $state<string | undefined>(undefined);
	loading = $state<boolean>(false);
	userId = $state<string>('');

	public init = async (userId: string) => {
		if (this.userId === userId) return;
		this.userId = userId;
		await this.reload();
	};

	public reload = async () => {
		if (!this.userId) return;
		this.loading = true;

		try {
			const { status, data: d, error: e } = await getUserHeatMap(this.userId);

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

export const userHeatMapState = new UserHeatMapState();
