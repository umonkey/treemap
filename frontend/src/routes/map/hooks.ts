// Loads data to show current state of the tree.

import { apiClient } from '$lib/api';
import { writable } from 'svelte/store';

export const mapState = () => {
	const center = writable<number[] | undefined>(undefined);
	const marker = writable<number[] | undefined>(undefined);

	const reload = async (tree_id: string | null) => {
		if (tree_id === null) {
			center.set(undefined);
			marker.set(undefined);
			return;
		}

		const res = await apiClient.getTree(tree_id);

		if (res.status === 200 && res.data) {
			const ll = [res.data.lat, res.data.lon];
			center.set(ll);
			marker.set(ll);

			console.debug(`[map] Center/Marker set to ${ll}`);
		}
	};

	return { center, marker, reload };
};
