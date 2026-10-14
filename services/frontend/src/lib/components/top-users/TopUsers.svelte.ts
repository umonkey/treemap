import { getTopUsers } from '$lib/api/stats';
import { type ITopUser } from '$lib/types';

export class TopUsers {
	users = $state<ITopUser[]>([]);
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
			const { status, data: d, error: e } = await getTopUsers();

			if (status === 200 && d) {
				this.users = d.users;
			}

			if (e) {
				this.error = e.description;
			}
		} finally {
			this.loading = false;
		}
	};
}

// Shortens large counts, e.g. 4555 -> "4.6 K".
export const formatCount = (count: number): string => {
	if (count > 1000) {
		return `${(count / 1000).toFixed(1)} K`;
	}

	return `${count}`;
};
