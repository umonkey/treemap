import { getWater, updateWaterLocation } from '$lib/api/water';
import { mapBus } from '$lib/buses/mapBus';
import { mapMode } from '$lib/stores/mapMode';
import { moveOriginState } from '$lib/stores/moveOriginState.svelte.ts';
import { goto, routes } from '$lib/routes';
import { roundCoord } from '$lib/utils/strings';
import { getDistance } from '$lib/utils';
import type { IWaterSource, ILatLng } from '$lib/types';

class PageState {
	source = $state<IWaterSource | undefined>(undefined);
	busy = $state(false);
	error = $state<string | undefined>(undefined);
	center = $state<ILatLng | undefined>(undefined);

	distance = $derived(
		this.source && moveOriginState.origin && this.center
			? getDistance(moveOriginState.origin, this.center)
			: 0
	);

	init = (id: string) => {
		this.busy = false;
		this.error = undefined;
		this.source = undefined;
		this.center = undefined;

		mapMode.set('move');
		mapBus.on('center', this.handleCenter);

		getWater(id).then((res) => {
			if (res.data) {
				this.source = res.data;
				this.center = { lat: res.data.lat, lng: res.data.lon };
				moveOriginState.origin = { lat: res.data.lat, lng: res.data.lon };
				mapBus.emit('move', { lat: res.data.lat, lng: res.data.lon });
			}
		});
	};

	destroy = () => {
		mapMode.set(undefined);
		moveOriginState.origin = undefined;
		mapBus.off('center', this.handleCenter);
	};

	private handleCenter = (ll: ILatLng) => {
		this.center = ll;
	};

	handleConfirm = async (id: string) => {
		if (!this.center) {
			return;
		}

		this.busy = true;
		this.error = undefined;

		const lat = roundCoord(this.center.lat);
		const lng = roundCoord(this.center.lng);

		const res = await updateWaterLocation(id, lat, lng);
		if (res.status >= 200 && res.status < 300) {
			goto(routes.waterDetails(id));
		} else {
			this.error = res.error?.description || 'Failed to move water source';
			this.busy = false;
		}
	};

	handleCancel = (id: string) => {
		goto(routes.waterDetails(id));
	};
}

export const pageState = new PageState();
