import {
	getPanorama,
	getPanoramaGeoJSON,
	getPanoramaHints,
	getPanoramasImage,
	type PanoramaImage
} from '$lib/api/panoramas';
import { mapRaysStore } from '$lib/stores/mapRays.svelte';
import { panoBus } from '$lib/buses/panoBus';
import { config } from '$lib/env';
import { showError } from '$lib/errors';
import { locale } from '$lib/locale';
import type { FeatureCollection } from 'geojson';
import { LngLatBounds, type Map } from 'maplibre-gl';

export class PanoramaPreviewState {
	geoJsonData = $state<FeatureCollection | undefined>(undefined);
	hintsGeoJsonData = $state<FeatureCollection | undefined>(undefined);
	loading = $state<boolean>(false);
	map = $state.raw<Map | undefined>(undefined);
	selectedImageId = $state<string | undefined>(undefined);
	selectedImage = $state<PanoramaImage | undefined>(undefined);
	loadingImage = $state<boolean>(false);
	yaw = $state<number>(0);
	private currentPanoramaId = $state<string | undefined>(undefined);

	layer = `https://api.maptiler.com/maps/openstreetmap/style.json?key=${config.mapTilerKey}&language=${locale.lang}`;

	public onMount = () => {
		panoBus.on('reload', this.handleReloadBus);
		return () => {
			panoBus.off('reload', this.handleReloadBus);
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

			addFeatures(this.geoJsonData);
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

	calculateDistance = (lat1: number, lon1: number, lat2: number, lon2: number): number => {
		const R = 6371000; // Earth radius in meters
		const toRad = (deg: number) => (deg * Math.PI) / 180;
		const dLat = toRad(lat2 - lat1);
		const dLon = toRad(lon2 - lon1);
		const a =
			Math.sin(dLat / 2) * Math.sin(dLat / 2) +
			Math.cos(toRad(lat1)) * Math.cos(toRad(lat2)) * Math.sin(dLon / 2) * Math.sin(dLon / 2);
		const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
		return R * c;
	};

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	handleMapClick = (e: any) => {
		if (!this.geoJsonData || !this.geoJsonData.features) return;
		const { lng, lat } = e.lngLat;
		let closestImageId: string | undefined = undefined;
		let minDistance = Number.POSITIVE_INFINITY;

		for (const feature of this.geoJsonData.features) {
			if (feature.properties?.kind === 'image' && feature.geometry.type === 'Point') {
				const [featureLng, featureLat] = feature.geometry.coordinates;
				if (!Number.isNaN(featureLat) && !Number.isNaN(featureLng)) {
					const distance = this.calculateDistance(lat, lng, featureLat, featureLng);
					if (distance < minDistance) {
						minDistance = distance;
						const imageId = feature.properties?.id ?? feature.id;
						if (imageId) {
							closestImageId = String(imageId);
						}
					}
				}
			}
		}

		if (closestImageId !== undefined && minDistance <= 2.0) {
			this.selectImage(closestImageId);
		}
	};

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	handleCircleClick = (e: any) => {
		const feature = e.features?.[0];
		if (!feature) return;
		const imageId = feature.properties?.id ?? feature.id;
		if (imageId) {
			this.selectImage(String(imageId));
		}
	};

	reload = async (panoramaId: string) => {
		this.currentPanoramaId = panoramaId;
		this.geoJsonData = undefined;
		this.hintsGeoJsonData = undefined;
		this.selectedImageId = undefined;
		this.selectedImage = undefined;
		this.yaw = 0;
		mapRaysStore.rays = [];
		this.loading = true;

		const [geoRes, hintsRes, panoRes] = await Promise.all([
			getPanoramaGeoJSON(panoramaId),
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

		if (geoRes.status === 200 && geoRes.data) {
			this.geoJsonData = geoRes.data as FeatureCollection;
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
		}

		if (this.geoJsonData || this.hintsGeoJsonData) {
			this.fitBounds();
			if (this.geoJsonData) {
				const firstImageFeature = this.geoJsonData.features.find(
					(f) => f.properties?.kind === 'image'
				);
				if (firstImageFeature) {
					const imageId = firstImageFeature.properties?.id ?? firstImageFeature.id;
					if (imageId) {
						this.selectImage(String(imageId));
					}
				}
			}
		} else {
			showError(geoRes.error?.description || 'Failed to load panorama preview');
		}
	};
}
