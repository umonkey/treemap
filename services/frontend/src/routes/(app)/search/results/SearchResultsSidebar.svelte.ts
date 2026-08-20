import { searchTrees } from '$lib/api/trees';
import { mapBus } from '$lib/buses/mapBus';
import { showError } from '$lib/errors';
import { goto, routes } from '$lib/routes';
import { mapZoom } from '$lib/stores/mapStore';
import { searchStore } from '$lib/stores/searchStore';
import type { ITree } from '$lib/types';
import { get } from 'svelte/store';

export class SearchResultsSidebarLogic {
	trees = $state<ITree[]>([]);
	selectedTreeId = $state<string | null>(null);
	loading = $state<boolean>(false);
	error = $state<string | null>(null);
	query = $state<string>('');

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

	reload = async (query: string, zoom?: number) => {
		this.query = query;
		this.error = null;

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
			const res = await searchTrees(query, zoom ?? get(mapZoom));
			if (res.status === 200 && res.data) {
				this.trees = res.data.trees.filter((t) => t.state !== 'placeholder');
				if (this.selectedTreeId && !this.trees.some((t) => t.id === this.selectedTreeId)) {
					this.selectedTreeId = null;
					mapBus.emit('pin', undefined);
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
		await goto(routes.search());
	};

	init = (query: string) => {
		this.selectedTreeId = null;
		this.query = query;
		if (query.trim()) {
			searchStore.set(query);
		}

		let initial = true;
		const unsubscribe = mapZoom.subscribe((newZoom) => {
			if (initial) {
				initial = false;
				return;
			}
			if (this.query.trim()) {
				void this.reload(this.query, newZoom);
			}
		});

		return () => {
			unsubscribe();
			searchStore.set(undefined);
			this.selectedTreeId = null;
			mapBus.emit('pin', undefined);
		};
	};
}
