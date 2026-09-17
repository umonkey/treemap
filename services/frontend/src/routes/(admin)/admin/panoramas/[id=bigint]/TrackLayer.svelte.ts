import { getPanoramaTrack, type PanoramaTrackPoint } from '$lib/api/panoramas';
import { showError } from '$lib/errors';
import type { FeatureCollection } from 'geojson';

export class TrackLayerState {
	trackGeoJson = $state<FeatureCollection | undefined>(undefined);
	loading = $state<boolean>(false);
	private currentPanoramaId = $state<string | undefined>(undefined);

	reload = async (panoramaId: string) => {
		this.currentPanoramaId = panoramaId;
		this.loading = true;

		const res = await getPanoramaTrack(panoramaId);
		this.loading = false;

		if (this.currentPanoramaId !== panoramaId) return;

		if (res.status === 200 && res.data) {
			this.trackGeoJson = this.toGeoJson(res.data);
		} else if (res.status === 404) {
			this.trackGeoJson = undefined;
		} else {
			this.trackGeoJson = undefined;
			showError(res.error?.description || 'Failed to load panorama track');
		}
	};

	private toGeoJson = (points: PanoramaTrackPoint[]): FeatureCollection | undefined => {
		if (points.length < 2) return undefined;

		return {
			type: 'FeatureCollection',
			features: [
				{
					type: 'Feature',
					geometry: {
						type: 'LineString',
						coordinates: points.map((p) => [p.lng, p.lat])
					},
					properties: {}
				}
			]
		};
	};
}
