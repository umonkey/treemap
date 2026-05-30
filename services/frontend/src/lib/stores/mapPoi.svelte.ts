import type { ILatLng } from '$lib/types';
import { getDistance } from '$lib/utils/geo';

export interface IMapPoi {
	lat: number;
	lon: number;
	url: string;
}

class MapPoiStore {
	trees = $state.raw<IMapPoi[]>([]);
	alerts = $state.raw<IMapPoi[]>([]);
	panoramas = $state.raw<IMapPoi[]>([]);

	pois = $derived.by(() => {
		return [...this.trees, ...this.alerts, ...this.panoramas];
	});

	getNearest = (center: ILatLng, maxDistance?: number) => {
		const pois = this.pois;
		if (!pois.length) {
			return undefined;
		}

		let minDistance = Infinity;
		let nearestPoi = null;

		for (const poi of pois) {
			const dist = getDistance(center, { lat: poi.lat, lng: poi.lon });
			if (dist < minDistance) {
				minDistance = dist;
				nearestPoi = poi;
			}
		}

		if (!nearestPoi || (maxDistance !== undefined && minDistance > maxDistance)) {
			return undefined;
		}

		return {
			poi: nearestPoi,
			distance: minDistance
		};
	};
}

export const mapPoiStore = new MapPoiStore();
