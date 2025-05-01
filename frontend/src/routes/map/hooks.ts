// Loads data to show current state of the tree.

import { apiClient } from '$lib/api';
import { writable } from 'svelte/store';
import { mapBus } from '$lib/buses';
import { routes, goto } from '$lib/routes';
import { setLastTree } from '$lib/stores/mapStore';
import type { ILatLng, MountFn, DestroyFn } from '$lib/types';

export const hooks = (mount: MountFn, destroy: DestroyFn) => {
	const pins = writable<ILatLng[]>([]);

	// This is called when the preview id changes in the URL.
	// We need to let the preview control know about this,
	// and also move the map to the new center.
	const handlePreviewChange = async (tree_id: string | null) => {
		console.debug(`[map] Preview id updated to ${tree_id}.`);

		// Remember last value, used by the navigation pane.
		setLastTree(tree_id);

		// No tree selected for preview.
		if (tree_id === null) {
			pins.set([]);
			return;
		}

		const res = await apiClient.getTree(tree_id);

		if (res.status === 200 && res.data) {
			mapBus.emit('center', [res.data.lat, res.data.lon]);

			pins.set([
				{
					lat: res.data.lat,
					lng: res.data.lon
				}
			]);
		}
	};

	// A tree was selected on the map.
	// This will update the page props, which will trigger handlePreviewChange,
	// which will center the tree and update the marker.
	const handleTreeClick = (id: string) => {
		console.debug(`[map] Received click on tree ${id}, updating preview.`);
		goto(routes.mapPreview(id));
	};

	mount(() => mapBus.on('select', handleTreeClick));
	destroy(() => mapBus.off('select', handleTreeClick));

	return { pins, handlePreviewChange };
};
