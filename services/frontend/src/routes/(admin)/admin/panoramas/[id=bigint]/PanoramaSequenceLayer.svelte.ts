import { getPanoramaGeoJSON } from '$lib/api/panoramas';
import { mapBus } from '$lib/buses/mapBus';
import { panoBus } from '$lib/buses/panoBus';
import { showError } from '$lib/errors';
import { onPageFocus } from '$lib/utils/onPageFocus';
import type { FeatureCollection } from 'geojson';
import { LngLatBounds } from 'maplibre-gl';

export class PanoramaSequenceLayerState {
	geoJsonData = $state<FeatureCollection | undefined>(undefined);
	loading = $state<boolean>(false);
	selectedImageId?: string;
	onSelectImage?: (id: string) => void;
	private currentPanoramaId = $state<string | undefined>(undefined);

	public init = () => {
		panoBus.on('reload', this.handleReloadBus);
		const cleanupFocus = onPageFocus(() => {
			if (this.currentPanoramaId && !this.loading) {
				void this.reload(this.currentPanoramaId);
			}
		});
		return () => {
			panoBus.off('reload', this.handleReloadBus);
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
		const res = await getPanoramaGeoJSON(panoramaId);
		this.loading = false;
		if (res.status === 200 && res.data) {
			const isFirstLoad = this.geoJsonData === undefined;
			this.geoJsonData = res.data as FeatureCollection;

			if (isFirstLoad) {
				this.fitBounds();
			}

			if (!this.selectedImageId) {
				const firstImageFeature = this.geoJsonData.features.find(
					(f) => f.properties?.kind === 'image'
				);
				const imageId = firstImageFeature?.properties?.id ?? firstImageFeature?.id;
				if (imageId) {
					this.onSelectImage?.(String(imageId));
				}
			}
		} else {
			this.geoJsonData = undefined;
			showError(res.error?.description || 'Failed to load panorama sequence');
		}
	};

	fitBounds = () => {
		if (!this.geoJsonData?.features) return;

		const bounds = new LngLatBounds();

		for (const feature of this.geoJsonData.features) {
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

		if (!bounds.isEmpty()) {
			const sw = bounds.getSouthWest();
			const ne = bounds.getNorthEast();
			mapBus.emit('fit', {
				start: { lat: sw.lat, lng: sw.lng },
				end: { lat: ne.lat, lng: ne.lng }
			});
		}
	};

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	handleCircleClick = (e: any) => {
		const feature = e.features?.[0];
		if (!feature) return;
		const imageId = feature.properties?.id ?? feature.id;
		if (imageId) {
			this.onSelectImage?.(String(imageId));
		}
	};
}
