import { addTree } from '$lib/api/trees';
import { mapBus } from '$lib/buses/mapBus';
import { mapState } from '$lib/components/map/MapLibre.svelte.ts';
import { showError } from '$lib/errors';
import { spreadDots } from '$lib/map';
import { goto, routes } from '$lib/routes';
import { mapRowState } from '$lib/stores/mapRowState.svelte';
import type { IAddTreesRequest, ILatLon } from '$lib/types';

class PageState {
	saving = $state<boolean>(false);

	setPointA = () => {
		const center = mapState.center;
		mapRowState.pointA = {
			lat: center.lat,
			lng: center.lng
		};
	};

	setPointB = () => {
		const center = mapState.center;
		mapRowState.pointB = {
			lat: center.lat,
			lng: center.lng
		};
	};

	handleCountChange = (value: number) => {
		mapRowState.count = value;
	};

	handleConfirm = async () => {
		if (!mapRowState.pointA || !mapRowState.pointB) {
			return;
		}

		const points: ILatLon[] = spreadDots(
			mapRowState.pointA,
			mapRowState.pointB,
			mapRowState.count
		).map((point) => ({
			lat: point.lat,
			lon: point.lng
		}));

		const req = {
			points,
			species: '',
			height: null,
			diameter: null,
			circumference: null,
			state: 'unknown',
			notes: null,
			year: null,
			files: []
		} as IAddTreesRequest;

		try {
			this.saving = true;

			const { status, data: d, error: e } = await addTree(req);

			if (status >= 200 && status < 300 && d) {
				const id = d.trees[0].id;
				goto(routes.mapPreview(id));
				mapRowState.init();
			} else if (e) {
				console.error(`Error ${status} adding trees.`, e);
				showError(e.description);
			}
		} finally {
			this.saving = false;
			mapBus.emit('reload');
		}
	};

	handleCancel = () => {
		mapRowState.init();
		goto(routes.map());
	};
}

export const pageState = new PageState();
