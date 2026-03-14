// Loads data to show current state of the tree.

import { apiClient } from '$lib/api';
import { mapBus } from '$lib/buses';
import { goto, routes } from '$lib/routes';
import { searchStore } from '$lib/stores';
import { mapPreviewStore } from '$lib/stores/mapPreviewStore';
import { menuState } from '$lib/stores/treeMenu';
import type { DestroyFn, ILatLng, MountFn } from '$lib/types';
import { writable } from 'svelte/store';
import { get } from 'svelte/store';

export const hooks = (mount: MountFn, destroy: DestroyFn) => {
	const pin = writable<ILatLng | null>(null);

	// This is called when the preview id changes in the URL.
	// We need to let the preview control know about this,
	// and also move the map to the new center.
	const handlePreviewChange = async (tree_id: string | null) => {
		console.debug(`[map] Preview id updated to ${tree_id}.`);
		mapPreviewStore.set(tree_id ?? undefined);

		// No tree selected for preview.
		if (tree_id === null) {
			pin.set(null);
			return;
		}

		const res = await apiClient.getTree(tree_id);

		if (res.status === 200 && res.data) {
			mapBus.emit('center', {
				lat: res.data.lat,
				lng: res.data.lon
			});

			pin.set({
				lat: res.data.lat,
				lng: res.data.lon
			});
		}
	};

	// A tree was selected on the map.
	// This will update the page props, which will trigger handlePreviewChange,
	// which will center the tree and update the marker.
	const handleTreeClick = (id: string) => {
		console.debug(`[map] Received click on tree ${id}, updating preview.`);
		goto(routes.mapPreview(id, get(searchStore)));
	};

	const handleTreeMenu = (id: string) => {
		console.debug(`[map] Received menu request for tree ${id}.`);
		goto(routes.mapPreview(id, get(searchStore)));
		menuState.set(true);
	};

	const handleSearchQuery = (query: string | null) => {
		searchStore.set(query ?? undefined);
	};

	const handleAddTree = (pos: ILatLng) => {
		goto(routes.treeAdd(pos.lat, pos.lng));
	};

	const handleAddRow = (start: ILatLng, end: ILatLng) => {
		goto(routes.addRow(start, end));
	};

	mount(() => {
		mapBus.on('select', handleTreeClick);
		mapBus.on('menu', handleTreeMenu);
	});

	destroy(() => {
		mapBus.off('select', handleTreeClick);
		mapBus.off('menu', handleTreeMenu);
		mapPreviewStore.set(undefined);
	});

	return {
		pin,
		handlePreviewChange,
		handleSearchQuery,
		handleAddTree,
		handleAddRow
	};
};
