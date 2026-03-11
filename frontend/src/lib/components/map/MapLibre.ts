import type { LngLatBounds } from 'maplibre-gl';
import { apiClient } from '$lib/api';
import { writable } from 'svelte/store';

type Properties = {
	id: string;
	state: string;
	crown: number;
	trunk: number;
};

type Geometry = {
	type: string;
	coordinates: number[];
};

type Feature = {
	type: string;
	id: string;
	geometry: Geometry;
	properties: Properties;
};

type Collection = {
	type: string;
	features: Feature[];
};

export const markers = writable<Collection | undefined>(undefined);

export const handleMoveEnd = (bounds: LngLatBounds) => {
	apiClient.getMarkers(bounds.getNorth(), bounds.getEast(), bounds.getSouth(), bounds.getNorth()).then(({ status, data }) => {
		if (status === 200 && data) {
			const features = data.trees.map((tree) => ({
				type: "Feature",
				id: tree.id,
				geometry: {
					type: "Point",
					coordinates: [tree.lon, tree.lat]
				},
				properties: {
					id: tree.id,
					state: tree.state,
					crown: tree.diameter ?? 0,
					trunk: tree.circumference ?? 0,
				},
			}));

			const GeoJSON = {
				type: 'FeatureCollection',
				features: features,
			};

			markers.set(GeoJSON);

			console.debug(`Loaded ${features.length} features.`, GeoJSON);
		}
	});
};
