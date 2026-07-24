import { getPanoramaTrackData, type ITrackPoint } from '$lib/api/panoramas';
import { config } from '$lib/env';
import { showError } from '$lib/errors';
import { locale } from '$lib/locale';
import type { FeatureCollection } from 'geojson';
import { LngLatBounds, type Map } from 'maplibre-gl';

class TrackPreviewState {
	trackData = $state<ITrackPoint[] | undefined>(undefined);
	loading = $state<boolean>(false);
	map = $state.raw<Map | undefined>(undefined);

	geoJson = $derived.by<FeatureCollection>(() => {
		if (!this.trackData) {
			return {
				type: 'FeatureCollection',
				features: []
			};
		}

		const coordinates = this.trackData.map((p) => [p.lng, p.lat]);
		const timestamps = this.trackData.map((p) => p.timestamp);
		const offsets = this.trackData.map((p) => p.offset);

		return {
			type: 'FeatureCollection',
			features: [
				{
					type: 'Feature',
					geometry: {
						type: 'LineString',
						coordinates
					},
					properties: {
						timestamps,
						offsets
					}
				}
			]
		};
	});

	layer = `https://api.maptiler.com/maps/positron/style.json?key=${config.mapTilerKey}&language=${locale.lang}`;

	constructor() {
		// Pure constructor
	}

	fitBounds = () => {
		if (!this.map || !this.trackData || this.trackData.length === 0) return;

		requestAnimationFrame(() => {
			if (!this.map || !this.trackData) return;

			this.map.resize();

			const bounds = new LngLatBounds();
			for (const p of this.trackData) {
				if (!isNaN(p.lat) && !isNaN(p.lng)) {
					bounds.extend([p.lng, p.lat]);
				}
			}

			if (!bounds.isEmpty()) {
				this.map.fitBounds(bounds, { padding: 20, animate: false });
			}
		});
	};

	reload = async (panoramaId: string) => {
		this.trackData = undefined;
		this.loading = true;
		const res = await getPanoramaTrackData(panoramaId);
		this.loading = false;
		if (res.status === 200 && res.data) {
			this.trackData = res.data;
			this.fitBounds();
		} else {
			showError(res.error?.description || 'Failed to load track data');
		}
	};
}

export const componentState = new TrackPreviewState();
