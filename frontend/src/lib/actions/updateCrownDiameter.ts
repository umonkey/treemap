import { updateTreeDiameter } from '$lib/api/trees';
import { showError } from '$lib/errors';
import { goto, routes } from '$lib/routes';
import { writable } from 'svelte/store';

export const updateCrownDiameter = (id: string) => {
	const busy = writable<boolean>(false);

	const handleConfirm = async (value: number) => {
		try {
			busy.set(true);
			await updateTreeDiameter(id, value);
			goto(routes.mapPreview(id));
		} catch (e) {
			console.error('Error updating tree canopy', e);
			showError('Error updating tree canopy.');
		} finally {
			busy.set(false);
		}
	};

	const handleCancel = () => {
		goto(routes.mapPreview(id));
	};

	return { busy, handleConfirm, handleCancel };
};
