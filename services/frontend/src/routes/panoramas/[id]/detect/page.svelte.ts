import {
	getMapillaryImage,
	getMapillaryImageTrees,
	addMapillaryTree,
	deleteMapillaryTrees,
	type MapillaryImage,
	type MapillaryTree
} from '$lib/api/mapillary';
import { goto, routes } from '$lib/routes';

class PageState {
	id = $state<string>('');
	image = $state<MapillaryImage | null>(null);
	trees = $state<MapillaryTree[]>([]);
	angle = $state(0);
	isBusy = $state(false);

	reload = async (id: string) => {
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
		}

		if (treesRes.data) {
			this.trees = treesRes.data;
		}
	};

	handleMove = (angle: number) => {
		this.angle = angle;
	};

	handleAddTree = async () => {
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
	};

	handleDeleteTrees = async () => {
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
	};

	handleClose = () => {
		goto(routes.panorama(this.id));
	};

	cleanup = () => {
		this.id = '';
		this.image = null;
		this.trees = [];
	};
}

export const pageState = new PageState();
