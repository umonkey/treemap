import type { IGcpWithRadius } from './MapPreview.svelte.ts';
import circle from '@turf/circle';
import type { FeatureCollection, Feature, Polygon } from 'geojson';

export class CircleLayerLogic {
	getCircleGeoJson(gcp: IGcpWithRadius): FeatureCollection | null {
		if (
			!gcp ||
			gcp.radius <= 0 ||
			Number.isNaN(gcp.lat) ||
			Number.isNaN(gcp.lng) ||
			(gcp.lat === 0 && gcp.lng === 0)
		) {
			return null;
		}

		try {
			const options = { steps: 64, units: 'meters' as const };
			const turfCircle: Feature<Polygon> = circle([gcp.lng, gcp.lat], gcp.radius, options);
			return {
				type: 'FeatureCollection',
				features: [turfCircle]
			};
		} catch {
			return null;
		}
	}
}
