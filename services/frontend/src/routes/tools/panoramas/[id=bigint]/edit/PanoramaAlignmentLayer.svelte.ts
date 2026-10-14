import type { FeatureCollection, Feature } from 'geojson';

export class PanoramaAlignmentLayerLogic {
	geoJson = $state<FeatureCollection | undefined>(undefined);
	hintsGeoJson = $state<FeatureCollection | undefined>(undefined);
	latOffset = $state<number>(0);
	lonOffset = $state<number>(0);
	baseLatOffset = $state<number>(0);
	baseLonOffset = $state<number>(0);

	shiftedGeoJson = $derived.by(() => {
		const features: Feature[] = [];

		const dLat = this.latOffset - this.baseLatOffset;
		const dLon = this.lonOffset - this.baseLonOffset;

		if (this.geoJson?.features) {
			for (const feature of this.geoJson.features) {
				const kind = feature.properties?.kind;
				if (kind === 'image' && feature.geometry.type === 'Point') {
					const [lng, lat] = feature.geometry.coordinates;
					const shiftedLat = lat + dLat;
					const shiftedLng = lng + dLon;

					features.push({
						...feature,
						geometry: {
							...feature.geometry,
							coordinates: [shiftedLng, shiftedLat]
						}
					});
				} else if (kind === 'sequence' || feature.geometry?.type === 'LineString') {
					const coords = (feature.geometry as unknown as { coordinates: number[][] }).coordinates;
					const shiftedCoords = coords.map(([lng, lat]) => [lng + dLon, lat + dLat]);
					features.push({
						...feature,
						geometry: {
							...feature.geometry,
							coordinates: shiftedCoords
						}
					} as Feature);
				} else {
					features.push(feature);
				}
			}
		}

		if (this.hintsGeoJson?.features) {
			for (const feature of this.hintsGeoJson.features) {
				if (feature.geometry?.type === 'LineString') {
					const coords = (feature.geometry as unknown as { coordinates: number[][] }).coordinates;
					const shiftedCoords = coords.map(([lng, lat]) => [
						lng + this.lonOffset,
						lat + this.latOffset
					]);
					features.push({
						...feature,
						geometry: {
							...feature.geometry,
							coordinates: shiftedCoords
						}
					} as Feature);
				} else {
					features.push(feature);
				}
			}
		}

		return {
			type: 'FeatureCollection' as const,
			features
		};
	});
}
