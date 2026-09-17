import { mapPoiStore } from '$lib/stores/mapPoi.svelte';
import { mapState } from './MapLibre.svelte.ts';
import { mapBus } from '$lib/buses/mapBus';
import type { ILatLng } from '$lib/types';

class ComponentState {
	center = $state<ILatLng>(mapState.center);
	maxDistance = $state<number>(100);

	nearestPoi = $derived(mapPoiStore.getNearest(this.center, this.maxDistance));

	links = $derived.by(() => {
		const result = this.nearestPoi;
		if (!result) {
			return [];
		}

		const center = this.center;
		const { poi, distance } = result;

		return [
			{
				poi,
				distance,
				midpoint: [(center.lng + poi.lon) / 2, (center.lat + poi.lat) / 2] as [number, number],
				line: {
					type: 'Feature' as const,
					geometry: {
						type: 'LineString' as const,
						coordinates: [
							[center.lng, center.lat],
							[poi.lon, poi.lat]
						]
					},
					properties: {}
				}
			}
		];
	});

	private handleCenter = (ll: ILatLng) => {
		this.center = ll;
	};

	public constructor() {
		mapBus.on('center', this.handleCenter);
	}
}

export const componentState = new ComponentState();
