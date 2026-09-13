import type { ILatLng } from '$lib/types';
import { getDistance } from '$lib/utils/geo';

export interface IMapPoi {
	lat: number;
	lon: number;
	url: string;
}

export interface INearestPoi {
	poi: IMapPoi;
	distance: number;
}

class MapPoiStore {
	trees = $state.raw<IMapPoi[]>([]);
	alerts = $state.raw<IMapPoi[]>([]);
	panoramas = $state.raw<IMapPoi[]>([]);
	water = $state.raw<IMapPoi[]>([]);

	pois = $derived.by(() => {
		return [...this.trees, ...this.alerts, ...this.panoramas, ...this.water];
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

	getNearestTrees = (center: ILatLng, count: number, maxDistance?: number): INearestPoi[] => {
		if (count <= 0) {
			return [];
		}
		const list: INearestPoi[] = [];
		for (const poi of this.trees) {
			const distance = getDistance(center, { lat: poi.lat, lng: poi.lon });
			if (maxDistance !== undefined && distance > maxDistance) {
				continue;
			}
			list.push({ poi, distance });
		}
		list.sort((a, b) => a.distance - b.distance);
		return list.slice(0, count);
	};
}

export const mapPoiStore = new MapPoiStore();
