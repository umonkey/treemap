import { spreadDots } from '$lib/map';
import type { ILatLng } from '$lib/types';

/**
 * Helper to create the GeoJSON for the row preview.
 */
function createPreviewGeoJSON(start: ILatLng, end: ILatLng, count: number) {
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	const res: any = {
		type: 'FeatureCollection',
		features: []
	};

	res.features.push({
		type: 'Feature',
		geometry: {
			type: 'LineString',
			coordinates: [
				[start.lng, start.lat],
				[end.lng, end.lat]
			]
		}
	});

	for (const dot of spreadDots(start, end, count)) {
		res.features.push({
			type: 'Feature',
			geometry: {
				type: 'Point',
				coordinates: [dot.lng, dot.lat]
			}
		});
	}

	return res;
}

class MapRowState {
	pointA = $state<ILatLng | null>(null);
	pointB = $state<ILatLng | null>(null);
	count = $state<number>(2);

	previewData = $derived.by(() => {
		// Explicitly access dependencies to ensure they are tracked by Svelte.
		const pointA = this.pointA;
		const pointB = this.pointB;
		const count = this.count;

		if (!pointA || !pointB) {
			return undefined;
		}

		return createPreviewGeoJSON(pointA, pointB, count);
	});

	reset = () => {
		this.pointA = null;
		this.pointB = null;
		this.count = 2;
	};
}

export const mapRowState = new MapRowState();
