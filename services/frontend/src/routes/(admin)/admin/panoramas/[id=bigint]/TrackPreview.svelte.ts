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

	getCoordinates = (offset: number): { lat: number; lng: number } | undefined => {
		if (!this.trackData || this.trackData.length === 0) return undefined;

		// 1. Find the index of the closest point before the current offset using findLastIndex.
		const index = this.trackData.findLastIndex((p) => p.offset <= offset);

		// 2. If before the first point, return the first point.
		if (index === -1) {
			return { lat: this.trackData[0].lat, lng: this.trackData[0].lng };
		}

		// 3. If at or after the last point, return the last point.
		if (index >= this.trackData.length - 1) {
			const last = this.trackData[this.trackData.length - 1];
			return { lat: last.lat, lng: last.lng };
		}

		// 4. Calculate the ratio between the two surrounding points based on the offset.
		const p1 = this.trackData[index];
		const p2 = this.trackData[index + 1];

		if (p2.offset === p1.offset) {
			return { lat: p1.lat, lng: p1.lng };
		}

		const ratio = (offset - p1.offset) / (p2.offset - p1.offset);

		// 5. Interpolate the lat and lng coordinates.
		const lat = p1.lat + (p2.lat - p1.lat) * ratio;
		const lng = p1.lng + (p2.lng - p1.lng) * ratio;

		return { lat, lng };
	};

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
