import { searchTrees } from '$lib/api/trees';
import { showError } from '$lib/errors';
import { goto, routes } from '$lib/routes';
import { mapZoom } from '$lib/stores/mapStore';
import { searchStore } from '$lib/stores/searchStore';
import type { ITree } from '$lib/types';
import { get } from 'svelte/store';

export class SearchResultsSidebarLogic {
	trees = $state<ITree[]>([]);
	loading = $state<boolean>(false);
	error = $state<string | null>(null);
	query = $state<string>('');

	reload = async (query: string, zoom?: number) => {
		this.query = query;
		this.error = null;

		if (!query.trim()) {
			this.trees = [];
			this.loading = false;
			searchStore.set(undefined);
			return;
		}

		searchStore.set(query);
		this.loading = true;

		try {
			const res = await searchTrees(query, zoom ?? get(mapZoom));
			if (res.status === 200 && res.data) {
				this.trees = res.data.trees.filter((t) => t.state !== 'placeholder');
			} else {
				this.trees = [];
				if (res.error) {
					this.error = res.error.description;
					showError(res.error.description);
				}
			}
		} catch (err) {
			this.trees = [];
			const message = err instanceof Error ? err.message : 'Failed to search trees';
			this.error = message;
			showError(message);
		} finally {
			this.loading = false;
		}
	};

	handleClose = async () => {
		searchStore.set(undefined);
		await goto(routes.search());
	};

	init = (query: string) => {
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
		};
	};
}
