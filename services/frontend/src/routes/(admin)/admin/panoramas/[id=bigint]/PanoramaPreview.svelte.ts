import { getPanoramaGeoJSON } from '$lib/api/panoramas';
import { config } from '$lib/env';
import { showError } from '$lib/errors';
import { locale } from '$lib/locale';
import type { FeatureCollection } from 'geojson';
import { LngLatBounds, type Map } from 'maplibre-gl';

export class PanoramaPreviewState {
	geoJsonData = $state<FeatureCollection | undefined>(undefined);
	loading = $state<boolean>(false);
	map = $state.raw<Map | undefined>(undefined);

	layer = `https://api.maptiler.com/maps/openstreetmap/style.json?key=${config.mapTilerKey}&language=${locale.lang}`;

	constructor() {
		// Pure constructor
	}

	fitBounds = () => {
		if (!this.map || !this.geoJsonData) return;

		requestAnimationFrame(() => {
			if (!this.map || !this.geoJsonData) return;

			this.map.resize();

			const bounds = new LngLatBounds();
			for (const feature of this.geoJsonData.features) {
				if (feature.geometry.type === 'Point') {
					const [lng, lat] = feature.geometry.coordinates;
					if (!isNaN(lat) && !isNaN(lng)) {
						bounds.extend([lng, lat]);
					}
				} else if (feature.geometry.type === 'LineString') {
					for (const coord of feature.geometry.coordinates) {
						const [lng, lat] = coord;
						if (!isNaN(lat) && !isNaN(lng)) {
							bounds.extend([lng, lat]);
						}
					}
				}
			}

			if (!bounds.isEmpty()) {
				this.map.fitBounds(bounds, { padding: 20, animate: false });
			}
		});
	};

	reload = async (panoramaId: string) => {
		this.geoJsonData = undefined;
		this.loading = true;
		const res = await getPanoramaGeoJSON(panoramaId);
		this.loading = false;
		if (res.status === 200 && res.data) {
			this.geoJsonData = res.data as FeatureCollection;
			this.fitBounds();
		} else {
			showError(res.error?.description || 'Failed to load panorama preview');
		}
	};
}
