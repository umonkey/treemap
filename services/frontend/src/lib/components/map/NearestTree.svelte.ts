import { mapPoiStore } from '$lib/stores/mapPoi.svelte';
import { mapState } from './MapLibre.svelte.ts';
import { mapBus } from '$lib/buses/mapBus';
import type { ILatLng } from '$lib/types';

class ComponentState {
	center = $state<ILatLng>(mapState.center);
	maxDistance = $state<number>(100);

	nearest = $derived.by(() => {
		const center = this.center;
		const result = mapPoiStore.getNearest(center, this.maxDistance);

		if (!result) {
			return undefined;
		}

		const { poi: nearestPoi, distance: minDistance } = result;

		return {
			poi: nearestPoi,
			distance: minDistance,
			midpoint: [(center.lng + nearestPoi.lon) / 2, (center.lat + nearestPoi.lat) / 2] as [
				number,
				number
			],
			line: {
				type: 'Feature' as const,
				geometry: {
					type: 'LineString' as const,
					coordinates: [
						[center.lng, center.lat],
						[nearestPoi.lon, nearestPoi.lat]
					]
				},
				properties: {}
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

export const componentState = new ComponentState();
