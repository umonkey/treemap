import {
	getPanorama,
	getPanoramaHints,
	getPanoramasImage,
	type PanoramaImage
} from '$lib/api/panoramas';
import { mapRaysStore } from '$lib/stores/mapRays.svelte';
import { panoBus } from '$lib/buses/panoBus';
import { config } from '$lib/env';
import { showError } from '$lib/errors';
import { locale } from '$lib/locale';
import { onPageFocus } from '$lib/utils/onPageFocus';
import type { FeatureCollection } from 'geojson';
import { LngLatBounds, type Map } from 'maplibre-gl';

export class PanoramaPreviewState {
	hintsGeoJsonData = $state<FeatureCollection | undefined>(undefined);
	loading = $state<boolean>(false);
	map = $state.raw<Map | undefined>(undefined);
	selectedImageId = $state<string | undefined>(undefined);
	selectedImage = $state<PanoramaImage | undefined>(undefined);
	loadingImage = $state<boolean>(false);
	yaw = $state<number>(0);
	private currentPanoramaId = $state<string | undefined>(undefined);

	layer = `https://api.maptiler.com/maps/openstreetmap/style.json?key=${config.mapTilerKey}&language=${locale.lang}`;

	public init = () => {
		panoBus.on('reload', this.handleReloadBus);
		const cleanupFocus = onPageFocus(() => {
			if (this.currentPanoramaId && !this.loading) {
				this.reload(this.currentPanoramaId, { preserveSelection: true });
			}
		});
		return () => {
			panoBus.off('reload', this.handleReloadBus);
			cleanupFocus();
		};
	};

	private handleReloadBus = () => {
		if (this.currentPanoramaId) {
			this.reload(this.currentPanoramaId);
		}
	};

	fitBounds = () => {
		if (!this.map) return;

		requestAnimationFrame(() => {
			if (!this.map) return;

			this.map.resize();

			const bounds = new LngLatBounds();
			const addFeatures = (fc?: FeatureCollection) => {
				if (!fc?.features) return;
				for (const feature of fc.features) {
					if (feature.geometry.type === 'Point') {
						const [lng, lat] = feature.geometry.coordinates;
						if (!Number.isNaN(lat) && !Number.isNaN(lng)) {
							bounds.extend([lng, lat]);
						}
					} else if (feature.geometry.type === 'LineString') {
						for (const coord of feature.geometry.coordinates) {
							const [lng, lat] = coord;
							if (!Number.isNaN(lat) && !Number.isNaN(lng)) {
								bounds.extend([lng, lat]);
							}
						}
					}
				}
			};

			addFeatures(this.hintsGeoJsonData);

			if (!bounds.isEmpty()) {
				this.map.fitBounds(bounds, { padding: 20, animate: false });
			}
		});
	};

	selectImage = async (imageId: string) => {
		this.selectedImageId = imageId;
		this.selectedImage = undefined;
		mapRaysStore.rays = [];
		this.loadingImage = true;
		const res = await getPanoramasImage(imageId);
		this.loadingImage = false;
		if (res.status === 200 && res.data) {
			this.selectedImage = res.data;
			const heading = (this.selectedImage.compass_angle + this.yaw + 360) % 360;
			mapRaysStore.rays = [
				{
					lat: this.selectedImage.lat,
					lng: this.selectedImage.lon,
					angle: heading,
					length: 20
				}
			];
		} else {
			showError(res.error?.description || 'Failed to load panorama image');
			mapRaysStore.rays = [];
		}
	};

	handleViewerMove = (angle: number) => {
		this.yaw = angle;
		if (this.selectedImage) {
			const heading = (this.selectedImage.compass_angle + this.yaw + 360) % 360;
			mapRaysStore.rays = [
				{
					lat: this.selectedImage.lat,
					lng: this.selectedImage.lon,
					angle: heading,
					length: 20
				}
			];
		}
	};

	reload = async (panoramaId: string, options?: { preserveSelection?: boolean }) => {
		this.currentPanoramaId = panoramaId;
		if (!options?.preserveSelection) {
			this.selectedImageId = undefined;
			this.selectedImage = undefined;
			this.yaw = 0;
			mapRaysStore.rays = [];
		}
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
			this.fitBounds();
		} else {
			this.hintsGeoJsonData = undefined;
			showError(hintsRes.error?.description || 'Failed to load panorama hints');
		}
	};
}
