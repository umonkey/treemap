import { getPanoramasImage, type PanoramaImage } from '$lib/api/panoramas';
import { mapBus } from '$lib/buses/mapBus';
import { mapRaysStore } from '$lib/stores/mapRays.svelte';
import { config } from '$lib/env';
import { showError } from '$lib/errors';
import { locale } from '$lib/locale';
import type { ILatLng } from '$lib/types';
import type { Map } from 'maplibre-gl';

export class PanoramaPreviewState {
	map = $state.raw<Map | undefined>(undefined);
	selectedImageId = $state<string | undefined>(undefined);
	selectedImage = $state<PanoramaImage | undefined>(undefined);
	loadingImage = $state<boolean>(false);
	yaw = $state<number>(0);

	layer = `https://api.maptiler.com/maps/openstreetmap/style.json?key=${config.mapTilerKey}&language=${locale.lang}`;

	handleMapLoad = () => {
		if (!this.map) return;

		requestAnimationFrame(() => {
			this.map?.resize();
		});
	};

	handleFit = ({ start, end }: { start: ILatLng; end: ILatLng }) => {
		if (!this.map) return;

		this.map.fitBounds(
			[
				[start.lng, start.lat],
				[end.lng, end.lat]
			],
			{ padding: 20, animate: false }
		);
	};

	public init = () => {
		mapBus.on('fit', this.handleFit);
		return () => {
			mapBus.off('fit', this.handleFit);
		};
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

	reload = (panoramaId: string) => {
		void panoramaId;
		this.selectedImageId = undefined;
		this.selectedImage = undefined;
		this.yaw = 0;
		mapRaysStore.rays = [];
	};
}
