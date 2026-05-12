import { mapBus } from '$lib/buses/mapBus';
import { mapState } from './MapLibre.svelte.ts';
import { moveOriginState } from '$lib/stores/moveOriginState.svelte.ts';
import type { ILatLng } from '$lib/types';

class MoveLineLogic {
	center = $state<ILatLng>();

	line = $derived.by(() => {
		const origin = moveOriginState.origin;
		const center = this.center || mapState.center;

		if (!origin || !center) {
			return undefined;
		}

		return {
			type: 'Feature' as const,
			properties: {},
			geometry: {
				type: 'LineString' as const,
				coordinates: [
					[origin.lng, origin.lat],
					[center.lng, center.lat]
				]
			}
		};
	});

	private handleCenter = (ll: ILatLng) => {
		this.center = ll;
	};

	public constructor() {
		mapBus.on('center', this.handleCenter);
	}
}

export const componentState = new MoveLineLogic();
