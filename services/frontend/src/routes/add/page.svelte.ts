import { addTree } from '$lib/api/trees';
import { mapBus } from '$lib/buses/mapBus';
import { mapState } from '$lib/components/map/MapLibre.svelte.ts';
import { showError } from '$lib/errors';
import { goto, routes } from '$lib/routes';
import type { IAddTreesRequest } from '$lib/types';

class PageState {
	saving = $state<boolean>(false);

	handleConfirm = () => {
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

		addTree(req)
			.then((res) => {
				if (res.status >= 200 && res.status < 400 && res.data) {
					const id = res.data.trees[0].id;
					goto(routes.treeEdit(id));
				} else {
					console.error(`Error ${res.status} adding tree.`, res);
					showError(res.error?.description || `Error ${res.status} adding tree.`);
				}
			})
			.finally(() => {
				this.saving = false;
				mapBus.emit('reload');
			});
	};

	handleCancel = () => {
		goto(routes.map());
	};
}

export const pageState = new PageState();
