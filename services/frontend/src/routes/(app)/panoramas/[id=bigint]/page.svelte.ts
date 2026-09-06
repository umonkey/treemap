import { goto, routes } from '$lib/routes';
import {
	getPanoramasImage,
	getPanoramasImageHints,
	addPanoramaImageHint,
	deleteImageHints,
	type PanoramaImage,
	type PanoramaHint
} from '$lib/api/panoramas';
import { mapRaysStore } from '$lib/stores/mapRays.svelte';
import { mapBus } from '$lib/buses/mapBus';
import { panoBus } from '$lib/buses/panoBus';

class PageState {
	id = $state<string>('');
	image = $state<PanoramaImage | null>(null);
	trees = $state<PanoramaHint[]>([]);
	angle = $state<number>(0);
	isBusy = $state(false);

	public handleClose = async () => {
		await goto(routes.home());
	};

	public handleTreeClick = async (treeId: string) => {
		await goto(routes.mapPreview(treeId));
	};

	public handleImageClick = async (imageId: string) => {
		await goto(routes.panorama(imageId));
	};

	public handleAddTree = async () => {
		if (!this.id || this.isBusy) return;

		const newTree: PanoramaHint = {
			angle: this.angle
		};

		// Optimistic update
		this.trees = [...this.trees, newTree];

		this.isBusy = true;
		const res = await addPanoramaImageHint(this.id, this.angle);
		if (res.error) {
			// Rollback optimistic update
			this.trees = this.trees.filter((t) => t !== newTree);
			this.isBusy = false;
			return;
		}

		// Refresh from server to get correct user_id if any
		const treesRes = await getPanoramasImageHints(this.id);
		if (treesRes.data) {
			this.trees = treesRes.data;
		}
		this.isBusy = false;
		panoBus.emit('reload');
	};

	public handleDeleteHints = async () => {
		if (!this.id || this.isBusy) return;

		this.isBusy = true;

		const res = await deleteImageHints(this.id);

		if (res.error) {
			this.isBusy = false;
			return;
		}

		// Only remove the manual hints, keep the auto-generated tree pointers
		// and sibling image pointers
		this.trees = this.trees.filter((t) => t.tree_id || t.image_id);

		this.isBusy = false;
		panoBus.emit('reload');
	};

	public handleMove = (angle: number) => {
		this.angle = angle;
		if (this.image) {
			mapRaysStore.rays = [
				{
					lat: this.image.lat,
					lng: this.image.lon,
					angle: (this.image.compass_angle + angle + 360) % 360
				}
			];
		}
	};

	public reload = async (id: string) => {
		if (this.id === id) return;
		this.id = id;
		this.image = null;
		this.trees = [];

		const [imageRes, treesRes] = await Promise.all([
			getPanoramasImage(id),
			getPanoramasImageHints(id)
		]);

		if (imageRes.data) {
			this.image = imageRes.data;

			const ll = { lat: this.image.lat, lng: this.image.lon };
			mapBus.emit('pin', ll);
			mapBus.emit('map-once', ll);
		}

		if (treesRes.data) {
			this.trees = treesRes.data;
		}
	};

	public cleanup = () => {
		this.id = '';
		this.image = null;
		this.trees = [];
		mapBus.emit('pin', undefined);
		mapRaysStore.rays = [];
	};
}

export const pageState = new PageState();
