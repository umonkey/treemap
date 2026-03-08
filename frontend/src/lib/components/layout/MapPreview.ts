import type { ITree, IObservation } from '$lib/types';
import { apiClient } from '$lib/api';
import { goto } from '$lib/routes';
import { mapHome } from '$lib/map';
import { menuState } from '$lib/stores/treeMenu';
import { mapPreviewStore } from '$lib/stores/mapPreviewStore';
import { writable } from 'svelte/store';

export const hook = () => {
	const visible = writable<boolean>(false);
	const tree = writable<ITree | null>(null);
	const error = writable<string | null>(null);
	const observations = writable<IObservation | null>(null);

	// This works by navigating to the map page which doesn't have ?preview=N in the address.
	const handleClose = (e: Event) => {
		e.preventDefault();
		goto(mapHome());
	};

	const reload = (id: string | null) => {
		console.debug(`[MapPreview] Selected tree: ${id}`);

		if (id === null) {
			visible.set(false);
			tree.set(null);
			return;
		}

		apiClient.getTree(id).then((res) => {
			if (res.status === 200 && res.data) {
				tree.set(res.data);
				visible.set(true);
			} else if (res.error) {
				error.set(res.error.description);
			}
		});

		apiClient.getObservations(id).then((res) => {
			if (res.status == 200) {
				observations.set(res.data ?? null);
			} else {
				observations.set(null);
			}
		});
	};

	mapPreviewStore.subscribe((value) => {
		reload(value ?? null);
	});

	const handleContextMenu = () => {
		menuState.set(true);
	};

	return { visible, error, tree, observations, handleClose, handleContextMenu };
};
