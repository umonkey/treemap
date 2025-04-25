import { toast } from '@zerodevx/svelte-toast';
import { apiClient } from '$lib/api';
import { writable } from 'svelte/store';
import { locale } from '$lib/locale';
import { goto, routes } from '$lib/routes';

export const updateCrownDiameter = (id: string) => {
	const busy = writable<boolean>(false);

	const handleConfirm = async (value: number) => {
		try {
			busy.set(true);
			await apiClient.updateTreeDiameter(id, value);
			toast.push(locale.measureCanopyUpdated());
			goto(routes.treeHistory(id));
		} catch (e) {
			console.error(`Error updating tree canopy: ${e}`);
			toast.push('Error updating tree canopy.');
		} finally {
			busy.set(false);
		}
	};

	const handleCancel = () => {
		goto(routes.treeHistory(id));
	};

	return { busy, handleConfirm, handleCancel };
};
