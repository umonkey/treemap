/**
 * Handle map change events.
 */

import { apiClient } from '$lib/api';

export const onMapMoveEnd = async (map, geoJSON) => {
	const n = map.getBounds().getNorth();
	const e = map.getBounds().getEast();
	const s = map.getBounds().getSouth();
	const w = map.getBounds().getWest();

	const res = await apiClient.getMarkers(n, e, s, w);

	if (res.status !== 200) {
		console.error('Failed to fetch markers', res);
		return;
	}

	console.info(`[map] Received ${res.data.trees.length} markers.`);

	const markers = res.data.trees.map((tree) => ({
		"type": "Point",
		"coordinates": [tree.lon, tree.lat]
	}));

	console.debug("MARKERS", markers);

	geoJSON.addData(markers);
};
