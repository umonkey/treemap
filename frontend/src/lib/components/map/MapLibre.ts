import type { LngLatBounds } from 'maplibre-gl';
import { apiClient } from '$lib/api';
import { type ITree } from '$lib/types';
import { get, writable } from 'svelte/store';
import { treeStore, addTrees } from '$lib/stores/treeStore';
import circle from '@turf/circle';

type Properties = {
	id: string;
	state: string;
	crown: number;
	trunk: number;
};

type Feature = {
	type: string;
	id: string;
	geometry: any;
	properties: Properties;
};

type Collection = {
	type: string;
	features: Feature[];
};

export const markers = writable<Collection | undefined>(undefined);

// Fetch debounce timer.
let fetchTimeout: ReturnType<typeof setTimeout>;

const fixCrown = (tree: ITree): number => {
	if (tree.state === 'gone' || tree.state === 'stump') {
		return 1.0;
	}

	const value = tree.diameter ?? 0;
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

const reloadTrees = (n: number, e: number, s: number, w: number) => {
	clearTimeout(fetchTimeout);

	fetchTimeout = setTimeout(() => {
		apiClient
			.getMarkers(n, e, s, w)
			.then(({ status, data }) => {
				if (status === 200 && data) {
					console.debug(`Received ${data.trees.length} trees.`);
					addTrees(data.trees);
				}
			});
	}, 1000);
}

export const handleMoveEnd = (bounds: LngLatBounds) => {
	reloadTrees(bounds.getNorth(), bounds.getEast(), bounds.getSouth(), bounds.getWest());
};

export const handleMoveStart = () => {
	clearTimeout(fetchTimeout);
};

const formatGeoJSON = (trees: ITree[]): Collection => {
	const features = trees.flatMap((tree) => {
		const features = [];

		const canopy = circle(
			[tree.lon, tree.lat],
			fixCrown(tree) / 2,
			{ units: "meters", steps: 16 },
		);

		features.push({
			type: 'Feature',
			id: `${tree.id}-canopy`,
			geometry: canopy.geometry,
			properties: {
				id: tree.id,
				type: "canopy",
				state: tree.state,
			}
		});

		const trunk = circle(
			[tree.lon, tree.lat],
			fixTrunk(tree.circumference ?? 0) / 2,
			{ units: "meters", steps: 16 },
		);

		features.push({
			type: 'Feature',
			id: `${tree.id}-trunk`,
			geometry: trunk.geometry,
			properties: {
				id: tree.id,
				type: "trunk",
				state: tree.state,
			}
		});

		return features;
	});

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
