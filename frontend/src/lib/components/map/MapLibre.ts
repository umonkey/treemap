import type { LngLatBounds } from 'maplibre-gl';
import { apiClient } from '$lib/api';
import { type ITree } from '$lib/types';
import { get, writable } from 'svelte/store';
import { treeStore, addTrees } from '$lib/stores/treeStore';

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

const fixCrown = (value: number | null): number => {
	return value ? value : 4;
};

const fixTrunk = (value: number | null): number => {
	const v = value ? value : 0.15;
	return v / 3.14159;
};

const updateGeoJSON = (trees: ITree[]) => {
	const items = Object.values(trees);
	const geoJSON = formatGeoJSON(items);
	markers.set(geoJSON);
	console.debug(`Updated GeoJSON with ${items.length} features.`);
};

export const handleMoveEnd = (bounds: LngLatBounds) => {
	updateGeoJSON(Object.values(get(treeStore)));

	apiClient
		.getMarkers(bounds.getNorth(), bounds.getEast(), bounds.getSouth(), bounds.getNorth())
		.then(({ status, data }) => {
			if (status === 200 && data) {
				addTrees(data.trees);
			}
		});
};

const getVisibleTrees = (trees: ITree[], bounds: LngLatBounds): ITree[] => {
	return trees.filter((t) => bounds.contains([t.lon, t.lat]));
};

const formatGeoJSON = (trees: ITree[]): Collection => {
	const features = trees.map((tree) => ({
		type: 'Feature',
		id: tree.id,
		geometry: {
			type: 'Point',
			coordinates: [tree.lon, tree.lat]
		},
		properties: {
			id: tree.id,
			state: tree.state,
			crown: fixCrown(tree.diameter ?? 0),
			trunk: fixTrunk(tree.circumference ?? 0)
		}
	}));

	return {
		type: 'FeatureCollection',
		features: features
	};
};

// Listen for tree cache updates, update markers.
export const handleMount = () => {
	const unsunscribe = treeStore.subscribe((trees) => {
		updateGeoJSON(Object.values(trees));
	});

	return () => {
		unsunscribe();
	};
};
