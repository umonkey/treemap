import { getUser } from '$lib/api/users';
import type { IUserWithRights, IError } from '$lib/types';

class PageState {
	user = $state<IUserWithRights | null>(null);
	loading = $state<boolean>(true);
	error = $state<IError | null>(null);
	currentId = $state<string>('');

	reload = async (id: string) => {
		if (this.currentId === id && this.user) return;
		this.currentId = id;
		this.loading = true;
		this.error = null;

		try {
			const res = await getUser(id);
			if (res.status === 200 && res.data) {
				this.user = res.data;
			} else {
				this.error = res.error || {
					code: 'load_failed',
					description: `Error ${res.status} loading user.`
				};
			}
		} catch (err) {
			console.error('Failed to reload user:', err);
			this.error = { code: 'exception', description: 'Failed to reload user.' };
		} finally {
			this.loading = false;
		}
	};
}

export const pageState = new PageState();
