import { toast } from '@zerodevx/svelte-toast';
import { apiClient } from '$lib/api';
import { writable } from 'svelte/store';
import { locale } from '$lib/locale';
import { goto, routes } from '$lib/routes';

export const markDead = (id: string) => {
	const busy = writable<boolean>(false);

	const handleConfirm = async () => {
		try {
			busy.set(true);
			await apiClient.updateTreeState(id, 'dead');
			toast.push(locale.deadNotification());
			goto(routes.treeHistory(id));
		} catch (e) {
			console.error(`Error deleting tree: ${e}`);
			toast.push('Error deleting tree.');
		} finally {
			busy.set(false);
		}
	};

	const handleCancel = () => {
		goto(routes.treeHistory(id));
	};

	return { busy, handleConfirm, handleCancel };
};
