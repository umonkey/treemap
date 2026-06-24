import { goto, routes } from '$lib/routes';
import {
	getMapillaryImage,
	getMapillaryImageTrees,
	addMapillaryTree,
	deleteMapillaryTrees,
	type MapillaryImage,
	type MapillaryTree
} from '$lib/api/mapillary';
import { mapRaysStore } from '$lib/stores/mapRays.svelte';
import { mapMarkerStore } from '$lib/stores/mapMarker.svelte';
import { mapBus } from '$lib/buses/mapBus';
import { panoBus } from '$lib/buses/panoBus';
import { LngLat } from 'maplibre-gl';

class PageState {
	id = $state<string>('');
	image = $state<MapillaryImage | null>(null);
	trees = $state<MapillaryTree[]>([]);
	angle = $state<number>(0);
	isBusy = $state(false);

	public handleClose = async () => {
		await goto(routes.home());
	};

	public handleAddTree = async () => {
		if (!this.id || this.isBusy) return;

		const newTree: MapillaryTree = {
			image_id: this.id,
			angle: this.angle,
			user_id: 0 // Will be set by backend
		};

		// Optimistic update
		this.trees = [...this.trees, newTree];

		this.isBusy = true;
		const res = await addMapillaryTree(this.id, this.angle);
		if (res.error) {
			// Rollback optimistic update
			this.trees = this.trees.filter((t) => t !== newTree);
			this.isBusy = false;
			return;
		}

		// Refresh from server to get correct user_id and tree_id if any
		const treesRes = await getMapillaryImageTrees(this.id);
		if (treesRes.data) {
			this.trees = treesRes.data;
		}
		this.isBusy = false;
		panoBus.emit('reload');
	};

	public handleDeleteTrees = async () => {
		if (!this.id || this.isBusy) return;

		const oldTrees = this.trees;
		this.trees = [];
		this.isBusy = true;

		const res = await deleteMapillaryTrees(this.id);
		if (res.error) {
			this.trees = oldTrees;
			this.isBusy = false;
			return;
		}

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
			getMapillaryImage(id),
			getMapillaryImageTrees(id)
		]);

		if (imageRes.data) {
			this.image = imageRes.data;

			const ll = { lat: this.image.lat, lng: this.image.lon };
			mapMarkerStore.center = new LngLat(ll.lng, ll.lat);
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
		mapMarkerStore.center = undefined;
		mapRaysStore.rays = [];
	};
}

export const pageState = new PageState();
