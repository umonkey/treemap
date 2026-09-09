import { getWater } from '$lib/api/water';
import { mapBus } from '$lib/buses/mapBus';
import { showError } from '$lib/errors';
import type { IWaterSource } from '$lib/types';

class PageState {
	source = $state<IWaterSource | undefined>(undefined);

	public reload = (id: string) => {
		this.source = undefined;

		getWater(id).then((res) => {
			if (res.status === 200 && res.data) {
				this.source = res.data;

				mapBus.emit('map-once', {
					lat: res.data.lat,
					lng: res.data.lon
				});
			} else if (res.error) {
				showError(res.error.description);
			}
		});
	};
}

export const pageState = new PageState();
