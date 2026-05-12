import { mapMode } from '$lib/stores/mapMode';
import { getTree, updateTreeLocation } from '$lib/api/trees';
import { mapBus } from '$lib/buses/mapBus';
import { mapState } from '$lib/components/map/MapLibre.svelte.ts';
import { moveOriginState } from '$lib/stores/moveOriginState.svelte.ts';
import { goto, routes } from '$lib/routes';
import { roundCoord } from '$lib/utils/strings';
import type { ITree } from '$lib/types';

class PageState {
	tree = $state<ITree | undefined>(undefined);
	busy = $state(false);
	error = $state<string | undefined>(undefined);

	init = (id: string) => {
		mapMode.set('move');

		getTree(id).then((res) => {
			if (res.data) {
				this.tree = res.data;
				moveOriginState.origin = { lat: res.data.lat, lng: res.data.lon };
				mapBus.emit('move', { lat: res.data.lat, lng: res.data.lon });
			}
		});
	};

	destroy = () => {
		mapMode.set(undefined);
		moveOriginState.origin = undefined;
	};

	handleConfirm = async (id: string) => {
		this.busy = true;
		this.error = undefined;

		const lat = roundCoord(mapState.center.lat);
		const lng = roundCoord(mapState.center.lng);

		const res = await updateTreeLocation(id, lat, lng);
		if (res.status >= 200 && res.status < 300) {
			goto(routes.mapPreview(id));
		} else {
			this.error = res.error?.description || 'Failed to move tree';
			this.busy = false;
		}
	};

	handleCancel = (id: string) => {
		goto(routes.mapPreview(id));
	};
}

export const pageState = new PageState();
