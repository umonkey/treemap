/**
 * Track my location and add to the current map.
 */

import { locationStore } from "$lib/stores/locationStore";
import L from "leaflet";

export const addLocateMeCircle = (map: L.Map) => {
	console.debug("[map] Adding my location display.");

	let circle: L.Layer | null = null;
	let dot: L.Layer | null = null;

	const unsubscribe = locationStore.subscribe((pos) => {
		console.debug("[map] My location changed:", pos);

		if (circle) {
			map.removeLayer(circle);
			circle = null;
		}

		if (dot) {
			map.removeLayer(dot);
			dot = null;
		}

		if (pos === null) {
			return;
		}

		circle = L.circle(pos, {
			radius: pos.accuracy,
			color: "#136AEC",
			fillColor: "#136AEC",
			fillOpacity: 0.15,
			weight: 0,
		}).addTo(map);

		dot = L.circleMarker(pos, {
			weight: 2,
			color: "#fff",
			fillColor: "#2A93EE",
			fillOpacity: 1,
			opacity: 1,
			radius: 9,
		}).addTo(map);
	});

	map.on("unload", () => {
		console.debug("[map] Removing my location display.");
		unsubscribe();
	});
};
