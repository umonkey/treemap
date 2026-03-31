import { apiClient } from '$lib/api';
import { showError } from '$lib/errors';
import { goto, routes } from '$lib/routes';
import { writable } from 'svelte/store';

export const markDead = (id: string) => {
	const busy = writable<boolean>(false);

	const handleConfirm = async () => {
		try {
			busy.set(true);
			await apiClient.updateTreeState(id, 'dead');
			goto(routes.mapPreview(id));
		} catch (e) {
			console.error('Error deleting tree', e);
			showError('Error deleting tree.');
		} finally {
			busy.set(false);
		}
	};

	const handleCancel = () => {
		goto(routes.mapPreview(id));
	};

	return { busy, handleConfirm, handleCancel };
};
