import { getUsers } from '$lib/api/users';
import type { IUserWithRights, IError } from '$lib/types';

class PageState {
	users = $state<IUserWithRights[]>([]);
	loading = $state<boolean>(true);
	error = $state<IError | null>(null);

	reload = async () => {
		this.loading = true;
		this.error = null;

		try {
			const res = await getUsers();
			if (res.status === 200 && res.data) {
				this.users = res.data.users;
			} else {
				this.error = res.error || {
					code: 'load_failed',
					description: `Error ${res.status} loading users.`
				};
			}
		} catch (err) {
			console.error('Failed to reload users:', err);
			this.error = { code: 'exception', description: 'Failed to reload users.' };
		} finally {
			this.loading = false;
		}
	};
}

export const pageState = new PageState();
