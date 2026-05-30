import { addTree } from '$lib/api/trees';
import { mapBus } from '$lib/buses/mapBus';
import { mapState } from '$lib/components/map/MapLibre.svelte.ts';
import { showError } from '$lib/errors';
import { goto, routes } from '$lib/routes';
import type { IAddTreesRequest } from '$lib/types';

class PageState {
	saving = $state<boolean>(false);

	private save = async (): Promise<string | null> => {
		const req = {
			points: [
				{
					lat: mapState.center.lat,
					lon: mapState.center.lng
				}
			],
			species: '',
			height: null,
			diameter: null,
			circumference: null,
			state: 'unknown',
			notes: null,
			year: null,
			files: []
		} as IAddTreesRequest;

		this.saving = true;

		try {
			const res = await addTree(req);
			if (res.status >= 200 && res.status < 400 && res.data) {
				const id = res.data.trees[0].id;
				if (typeof navigator !== 'undefined' && navigator.vibrate) {
					navigator.vibrate(50);
				}
				return id;
			} else {
				console.error(`Error ${res.status} adding tree.`, res);
				showError(res.error?.description || `Error ${res.status} adding tree.`);
				return null;
			}
		} finally {
			this.saving = false;
			mapBus.emit('reload');
		}
	};

	handleConfirm = async () => {
		const id = await this.save();
		if (id) {
			goto(routes.treeEdit(id));
		}
	};

	handleQuickAdd = async () => {
		const id = await this.save();
		if (id) {
			goto(routes.treeAdd());
		}
	};

	handleCancel = () => {
		goto(routes.map());
	};
}

export const pageState = new PageState();
