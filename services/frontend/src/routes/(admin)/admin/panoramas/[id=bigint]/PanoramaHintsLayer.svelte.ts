import { getPanorama, getPanoramaHints } from '$lib/api/panoramas';
import { panoBus } from '$lib/buses/panoBus';
import { showError } from '$lib/errors';
import { onPageFocus } from '$lib/utils/onPageFocus';
import type { FeatureCollection } from 'geojson';

export class PanoramaHintsLayerState {
	hintsGeoJsonData = $state<FeatureCollection | undefined>(undefined);
	loading = $state<boolean>(false);
	private currentPanoramaId = $state<string | undefined>(undefined);

	public init = () => {
		panoBus.on('reload', this.handleReloadBus);
		panoBus.on('reloadHints', this.handleReloadBus);
		const cleanupFocus = onPageFocus(() => {
			if (this.currentPanoramaId && !this.loading) {
				void this.reload(this.currentPanoramaId);
			}
		});
		return () => {
			panoBus.off('reload', this.handleReloadBus);
			panoBus.off('reloadHints', this.handleReloadBus);
			cleanupFocus();
		};
	};

	private handleReloadBus = () => {
		if (this.currentPanoramaId && !this.loading) {
			void this.reload(this.currentPanoramaId);
		}
	};

	reload = async (panoramaId: string) => {
		this.currentPanoramaId = panoramaId;
		this.loading = true;

		const [hintsRes, panoRes] = await Promise.all([
			getPanoramaHints(panoramaId),
			getPanorama(panoramaId)
		]);
		this.loading = false;

		let latOffset = 0;
		let lonOffset = 0;
		if (panoRes.status === 200 && panoRes.data) {
			latOffset = panoRes.data.lat_offset;
			lonOffset = panoRes.data.lon_offset;
		}

		if (hintsRes.status === 200 && hintsRes.data) {
			const rawHints = hintsRes.data as FeatureCollection;
			const shiftedFeatures = rawHints.features.map((feature) => {
				if (feature.geometry?.type === 'LineString') {
					const coords = feature.geometry.coordinates as [number, number][];
					const shiftedCoords = coords.map(([lng, lat]) => [lng + lonOffset, lat + latOffset]);
					return {
						...feature,
						geometry: {
							...feature.geometry,
							coordinates: shiftedCoords
						}
					};
				}
				return feature;
			});
			this.hintsGeoJsonData = {
				type: 'FeatureCollection',
				features: shiftedFeatures
			};
		} else {
			this.hintsGeoJsonData = undefined;
			showError(hintsRes.error?.description || 'Failed to load panorama hints');
		}
	};
}
