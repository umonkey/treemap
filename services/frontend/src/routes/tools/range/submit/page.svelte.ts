import { routes, goto } from '$lib/routes';
import { rangeStore, type ITriangulatedTree } from '../store.svelte';
import { addTree } from '$lib/api/trees';
import { mapBus } from '$lib/buses/mapBus';
import { showError } from '$lib/errors';
import type { IAddTreesRequest, ILatLon } from '$lib/types';

export class RangeSubmitState {
	trees = $state<ITriangulatedTree[]>([]);
	submitting = $state(false);

	constructor() {
		if (rangeStore.trees.length === 0) {
			goto(routes.toolsRangeEnter());
			return;
		}
		this.trees = rangeStore.trees;
	}

	handleBack = () => {
		goto(routes.toolsRangeEnter());
	};

	handleSubmit = async () => {
		if (this.submitting || this.trees.length === 0) return;

		const points: ILatLon[] = this.trees.map((t) => ({
			lat: t.lat,
			lon: t.lng
		}));

		const req: IAddTreesRequest = {
			points,
			species: 'unknown',
			notes: null,
			height: null,
			circumference: null,
			diameter: null,
			year: null,
			state: 'unknown',
			files: []
		};

		this.submitting = true;

		try {
			const res = await addTree(req);
			if (res.status >= 200 && res.status < 400 && res.data?.trees?.[0]?.id) {
				const id = res.data.trees[0].id;
				rangeStore.clearTrees();
				mapBus.emit('reload');
				goto(routes.mapPreview(id));
			} else {
				console.error(`Error ${res.status} adding triangulated trees.`, res);
				showError(res.error?.description || `Error ${res.status} adding trees.`);
				this.submitting = false;
			}
		} catch (err) {
			console.error('Exception adding triangulated trees', err);
			showError('An unexpected error occurred while submitting trees.');
			this.submitting = false;
		}
	};
}
