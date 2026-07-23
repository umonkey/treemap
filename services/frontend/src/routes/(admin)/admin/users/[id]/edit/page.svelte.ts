import { getUser, updateUser } from '$lib/api/users';
import { showError } from '$lib/errors';
import { goto } from '$app/navigation';
import type { IUser, IError } from '$lib/types';

class PageState {
	user = $state<IUser | null>(null);
	loading = $state<boolean>(true);
	saving = $state<boolean>(false);
	error = $state<IError | null>(null);
	currentId = $state<string>('');

	// Form state
	name = $state<string>('');

	reload = async (id: string) => {
		if (this.currentId === id && this.user) return;
		this.currentId = id;
		this.loading = true;
		this.error = null;

		try {
			const res = await getUser(id);
			if (res.status === 200 && res.data) {
				this.user = res.data.user;
				this.name = res.data.user.name;
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

	handleSave = async () => {
		if (!this.user) return;
		this.saving = true;

		try {
			const res = await updateUser(this.user.id, { name: this.name });
			if (res.status >= 200 && res.status < 300) {
				goto('/admin/users');
			} else {
				showError(res.error?.description || `Error ${res.status} updating user.`);
			}
		} catch (err) {
			console.error('Failed to update user:', err);
			showError('Failed to update user.');
		} finally {
			this.saving = false;
		}
	};
}

export const pageState = new PageState();
