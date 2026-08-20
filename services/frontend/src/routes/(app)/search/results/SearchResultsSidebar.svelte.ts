import { getSearchTreesCSV, searchTrees } from '$lib/api/trees';
import { mapBus } from '$lib/buses/mapBus';
import { showError } from '$lib/errors';
import { goto, routes } from '$lib/routes';
import { mapStore } from '$lib/stores/mapStore';
import { searchStore } from '$lib/stores/searchStore';
import type { IBounds, ITree } from '$lib/types';
import { get } from 'svelte/store';

export class SearchResultsSidebarLogic {
	trees = $state<ITree[]>([]);
	selectedTreeId = $state<string | null>(null);
	loading = $state<boolean>(false);
	error = $state<string | null>(null);
	query = $state<string>('');
	bounds: IBounds | undefined = undefined;

	getDownloadUrl = () => {
		return getSearchTreesCSV(this.query, get(mapStore).zoom, this.bounds);
	};

	selectTree = (tree: ITree) => {
		this.selectedTreeId = tree.id;
		const ll = { lat: tree.lat, lng: tree.lon };
		mapBus.emit('move', ll);
		mapBus.emit('pin', ll);
	};

	navigateToPreview = async (e: Event, treeId: string) => {
		e.preventDefault();
		e.stopPropagation();
		await goto(routes.mapPreview(treeId));
	};

	reload = async (query: string, zoom?: number, bounds?: IBounds) => {
		if (query !== this.query) {
			this.bounds = undefined;
		}
		if (bounds) {
			this.bounds = bounds;
		}
		this.query = query;
		this.error = null;

		const initialBounds = this.bounds;

		if (!query.trim()) {
			this.trees = [];
			this.selectedTreeId = null;
			this.loading = false;
			searchStore.set(undefined);
			mapBus.emit('pin', undefined);
			return;
		}

		searchStore.set(query);
		this.loading = true;

		try {
			const res = await searchTrees(query, zoom ?? get(mapStore).zoom, this.bounds);
			if (res.status === 200 && res.data) {
				this.trees = res.data.trees.filter((t) => t.state !== 'placeholder');
				if (this.selectedTreeId && !this.trees.some((t) => t.id === this.selectedTreeId)) {
					this.selectedTreeId = null;
					mapBus.emit('pin', undefined);
				}
				if (!initialBounds && this.trees.length > 0) {
					let minLat = Number.POSITIVE_INFINITY;
					let maxLat = Number.NEGATIVE_INFINITY;
					let minLon = Number.POSITIVE_INFINITY;
					let maxLon = Number.NEGATIVE_INFINITY;
					for (const t of this.trees) {
						if (t.lat < minLat) minLat = t.lat;
						if (t.lat > maxLat) maxLat = t.lat;
						if (t.lon < minLon) minLon = t.lon;
						if (t.lon > maxLon) maxLon = t.lon;
					}
					if (minLat !== Number.POSITIVE_INFINITY) {
						mapBus.emit('fit', {
							start: { lat: minLat, lng: minLon },
							end: { lat: maxLat, lng: maxLon }
						});
					}
				}
			} else {
				this.trees = [];
				this.selectedTreeId = null;
				mapBus.emit('pin', undefined);
				if (res.error) {
					this.error = res.error.description;
					showError(res.error.description);
				}
			}
		} catch (err) {
			this.trees = [];
			this.selectedTreeId = null;
			mapBus.emit('pin', undefined);
			const message = err instanceof Error ? err.message : 'Failed to search trees';
			this.error = message;
			showError(message);
		} finally {
			this.loading = false;
		}
	};

	handleClose = async () => {
		this.selectedTreeId = null;
		searchStore.set(undefined);
		mapBus.emit('pin', undefined);
		await goto(routes.home());
	};

	handleBounds = (bounds: IBounds) => {
		this.bounds = bounds;
		if (this.query.trim()) {
			void this.reload(this.query, undefined, bounds);
		}
	};

	init = () => {
		this.selectedTreeId = null;
		this.bounds = undefined;
		mapBus.on('bounds', this.handleBounds);

		return () => {
			mapBus.off('bounds', this.handleBounds);
			this.selectedTreeId = null;
			this.bounds = undefined;
			mapBus.emit('pin', undefined);
		};
	};
}
