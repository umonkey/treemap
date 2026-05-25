import { treeLayerState } from './TreeLayer.svelte.ts';
import { mapState } from './MapLibre.svelte.ts';
import { getDistance } from '$lib/utils/geo';
import { mapBus } from '$lib/buses/mapBus';
import type { ILatLng } from '$lib/types';

class ComponentState {
	center = $state<ILatLng>(mapState.center);

	nearest = $derived.by(() => {
		const center = this.center;
		const collection = treeLayerState.markers;
		if (!collection || !collection.features.length) {
			return undefined;
		}

		let minDistance = Infinity;
		let nearestFeature = null;

		for (const feature of collection.features) {
			const [lng, lat] = feature.geometry.coordinates;
			const dist = getDistance(center, { lat, lng });
			if (dist < minDistance) {
				minDistance = dist;
				nearestFeature = feature;
			}
		}

		// Only show the line if the distance is reasonable (e.g. within 100 meters).
		if (!nearestFeature || minDistance > 100) {
			return undefined;
		}

		return {
			feature: nearestFeature,
			distance: minDistance,
			midpoint: [
				(center.lng + nearestFeature.geometry.coordinates[0]) / 2,
				(center.lat + nearestFeature.geometry.coordinates[1]) / 2
			] as [number, number],
			line: {
				type: 'Feature' as const,
				geometry: {
					type: 'LineString' as const,
					coordinates: [[center.lng, center.lat], nearestFeature.geometry.coordinates]
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
