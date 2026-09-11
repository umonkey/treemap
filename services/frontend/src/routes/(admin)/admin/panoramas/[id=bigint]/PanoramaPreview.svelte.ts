import {
	getPanoramasImage,
	getPanoramasImageHints,
	addPanoramaImageHint,
	deleteImageHints,
	type PanoramaImage,
	type PanoramaHint
} from '$lib/api/panoramas';
import { mapBus } from '$lib/buses/mapBus';
import { panoBus } from '$lib/buses/panoBus';
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
	hints = $state<PanoramaHint[]>([]);
	loadingImage = $state<boolean>(false);
	isBusy = $state<boolean>(false);
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
		this.hints = [];
		mapRaysStore.rays = [];
		this.loadingImage = true;
		const [res, hintsRes] = await Promise.all([
			getPanoramasImage(imageId),
			getPanoramasImageHints(imageId)
		]);
		this.loadingImage = false;
		if (res.status === 200 && res.data) {
			this.selectedImage = res.data;
			if (hintsRes.data) {
				this.hints = hintsRes.data;
			}
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

	handleAddHint = async () => {
		if (!this.selectedImageId || this.isBusy) return;

		const newHint: PanoramaHint = { angle: this.yaw };
		this.hints = [...this.hints, newHint];

		this.isBusy = true;
		const res = await addPanoramaImageHint(this.selectedImageId, this.yaw);

		if (res.error) {
			this.hints = this.hints.filter((h) => h !== newHint);
			this.isBusy = false;
			showError(res.error.description || 'Failed to add hint');
			return;
		}

		const hintsRes = await getPanoramasImageHints(this.selectedImageId);
		if (hintsRes.data) {
			this.hints = hintsRes.data;
		}
		this.isBusy = false;

		panoBus.emit('reload');
	};

	handleDeleteHints = async () => {
		if (!this.selectedImageId || this.isBusy) return;

		this.isBusy = true;
		const res = await deleteImageHints(this.selectedImageId);

		if (res.error) {
			this.isBusy = false;
			showError(res.error.description || 'Failed to delete hints');
			return;
		}

		// Only remove the manual hints, keep the auto-generated tree pointers
		// and sibling image pointers
		this.hints = this.hints.filter((h) => h.tree_id || h.image_id);

		this.isBusy = false;
		panoBus.emit('reload');
	};

	reload = (panoramaId: string) => {
		void panoramaId;
		this.selectedImageId = undefined;
		this.selectedImage = undefined;
		this.hints = [];
		this.yaw = 0;
		mapRaysStore.rays = [];
	};
}
