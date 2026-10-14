import {
	getPanorama,
	getPanoramaGeoJSON,
	getPanoramaHints,
	type Panorama
} from '$lib/api/panoramas';
import { getOffsetDelta, roundOffset } from '$lib/utils/geo';
import { showError } from '$lib/errors';
import type { FeatureCollection } from 'geojson';
import type { Map as MapLibreGl } from 'maplibre-gl';

export class PanoramaAlignerLogic {
	panoramaId = $state<string>('');
	latOffset = $state<number>(0);
	lonOffset = $state<number>(0);
	baseLatOffset = $state<number>(0);
	baseLonOffset = $state<number>(0);

	geoJson = $state<FeatureCollection | undefined>(undefined);
	hintsGeoJson = $state<FeatureCollection | undefined>(undefined);
	panorama = $state<Panorama | undefined>(undefined);
	isLoading = $state<boolean>(false);
	map = $state.raw<MapLibreGl | undefined>(undefined);

	centerLat = $derived.by(() => {
		const first = this.geoJson?.features?.[0];
		const geom = first?.geometry as { coordinates?: unknown } | undefined;
		const coords = geom?.coordinates;
		if (Array.isArray(coords)) {
			if (typeof coords[1] === 'number') return coords[1];
			if (Array.isArray(coords[0]) && typeof coords[0][1] === 'number') return coords[0][1];
		}
		return 40.1872;
	});

	reload = async (id: string) => {
		if (!id) return;
		this.panoramaId = id;
		this.isLoading = true;
		try {
			const [panoRes, geoRes, hintsRes] = await Promise.all([
				getPanorama(id),
				getPanoramaGeoJSON(id),
				getPanoramaHints(id)
			]);

			if (panoRes.status === 200 && panoRes.data) {
				this.panorama = panoRes.data;
				this.baseLatOffset = roundOffset(panoRes.data.lat_offset);
				this.baseLonOffset = roundOffset(panoRes.data.lon_offset);
				if (this.latOffset === 0 && this.lonOffset === 0) {
					this.latOffset = roundOffset(panoRes.data.lat_offset);
					this.lonOffset = roundOffset(panoRes.data.lon_offset);
				}
			}

			if (geoRes.status === 200 && geoRes.data) {
				this.geoJson = geoRes.data as FeatureCollection;
			}

			if (hintsRes.status === 200 && hintsRes.data) {
				this.hintsGeoJson = hintsRes.data as FeatureCollection;
			}

			this.fitBounds();
		} catch (e) {
			console.error('Error loading panorama aligner data', e);
			showError('Failed to load panorama alignment data.');
		} finally {
			this.isLoading = false;
		}
	};

	fitBounds = () => {
		if (!this.map) return;

		requestAnimationFrame(() => {
			if (!this.map) return;
			this.map.resize();

			try {
				let minLat = Number.POSITIVE_INFINITY;
				let maxLat = Number.NEGATIVE_INFINITY;
				let minLng = Number.POSITIVE_INFINITY;
				let maxLng = Number.NEGATIVE_INFINITY;

				// eslint-disable-next-line @typescript-eslint/no-explicit-any
				const processCoord = (c: any) => {
					if (
						Array.isArray(c) &&
						c.length >= 2 &&
						typeof c[0] === 'number' &&
						typeof c[1] === 'number'
					) {
						const [lng, lat] = c;
						if (lng < minLng) minLng = lng;
						if (lng > maxLng) maxLng = lng;
						if (lat < minLat) minLat = lat;
						if (lat > maxLat) maxLat = lat;
					} else if (Array.isArray(c)) {
						for (const sub of c) {
							processCoord(sub);
						}
					}
				};

				if (this.geoJson?.features) {
					for (const f of this.geoJson.features) {
						processCoord((f.geometry as { coordinates?: unknown })?.coordinates);
					}
				}

				if (this.hintsGeoJson?.features) {
					for (const f of this.hintsGeoJson.features) {
						processCoord((f.geometry as { coordinates?: unknown })?.coordinates);
					}
				}

				if (minLat !== Number.POSITIVE_INFINITY) {
					this.map.fitBounds([minLng, minLat, maxLng, maxLat], {
						padding: 50,
						maxZoom: 19,
						duration: 500
					});
				}
			} catch (e) {
				console.error('Error fitting bounds', e);
			}
		});
	};

	nudge = (direction: 'up' | 'down' | 'left' | 'right') => {
		const { deltaLat, deltaLon } = getOffsetDelta(this.centerLat, 0.1);
		switch (direction) {
			case 'up':
				this.latOffset = roundOffset(this.latOffset + deltaLat);
				break;
			case 'down':
				this.latOffset = roundOffset(this.latOffset - deltaLat);
				break;
			case 'right':
				this.lonOffset = roundOffset(this.lonOffset + deltaLon);
				break;
			case 'left':
				this.lonOffset = roundOffset(this.lonOffset - deltaLon);
				break;
		}
	};

	handleKeydown = (e: KeyboardEvent) => {
		if (e.ctrlKey || e.metaKey || e.altKey) {
			return;
		}
		const target = e.target as HTMLElement;
		if (['INPUT', 'TEXTAREA', 'BUTTON'].includes(target?.tagName)) {
			return;
		}
		switch (e.key) {
			case 'ArrowUp':
			case 'w':
			case 'W':
				e.preventDefault();
				this.nudge('up');
				break;
			case 'ArrowDown':
			case 's':
			case 'S':
				e.preventDefault();
				this.nudge('down');
				break;
			case 'ArrowLeft':
			case 'a':
			case 'A':
				e.preventDefault();
				this.nudge('left');
				break;
			case 'ArrowRight':
			case 'd':
			case 'D':
				e.preventDefault();
				this.nudge('right');
				break;
		}
	};
}
