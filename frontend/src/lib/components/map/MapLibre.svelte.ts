import type { LngLatBounds } from 'maplibre-gl';
import { apiClient } from '$lib/api';
import { type ITree } from '$lib/types';
import { treeStore, addTrees } from '$lib/stores/treeStore';
import { mapStore } from '$lib/stores/mapStore';
import { get } from 'svelte/store';
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

class MapLibre {
	markers = $state<Collection | undefined>(undefined);
	zoom = $state<number>(13);

	fetchTimeout: ReturnType<typeof setTimeout> | undefined = undefined;

	public constructor() {
		this.zoom = get(mapStore)?.zoom ?? 13;
	}

	private fixCrown(tree: ITree): number {
		if (tree.state === 'gone' || tree.state === 'stump') {
			return 1.0;
		}

		const value = tree.diameter ?? 0;
		return value ? value : 4;
	}

	private fixTrunk(value: number | null): number {
		const v = value ? value : 0.15;
		return v / 3.14159;
	}

	private updateGeoJSON(trees: ITree[]) {
		const items = Object.values(trees);
		this.markers = this.formatGeoJSON(items);
		console.debug(`Updated GeoJSON with ${items.length} features.`);
	}

	private reloadTrees(n: number, e: number, s: number, w: number) {
		clearTimeout(this.fetchTimeout);

		this.fetchTimeout = setTimeout(() => {
			apiClient.getMarkers(n, e, s, w).then(({ status, data }) => {
				if (status === 200 && data) {
					console.debug(`Received ${data.trees.length} trees.`);
					addTrees(data.trees);
				}
			});
		}, 1000);
	}

	public handleMoveEnd(bounds: LngLatBounds) {
		this.reloadTrees(bounds.getNorth(), bounds.getEast(), bounds.getSouth(), bounds.getWest());
	}

	public handleMoveStart() {
		clearTimeout(this.fetchTimeout);
	}

	private formatGeoJSON(trees: ITree[]): Collection {
		const features = trees.flatMap((tree) => {
			const features = [];

			const canopy = circle([tree.lon, tree.lat], this.fixCrown(tree) / 2, {
				units: 'meters',
				steps: 16
			});

			features.push({
				type: 'Feature',
				id: `${tree.id}-canopy`,
				geometry: canopy.geometry,
				properties: {
					id: tree.id,
					type: 'canopy',
					state: tree.state
				}
			});

			const trunk = circle([tree.lon, tree.lat], this.fixTrunk(tree.circumference ?? 0) / 2, {
				units: 'meters',
				steps: 16
			});

			features.push({
				type: 'Feature',
				id: `${tree.id}-trunk`,
				geometry: trunk.geometry,
				properties: {
					id: tree.id,
					type: 'trunk',
					state: tree.state
				}
			});

			return features;
		});

		return {
			type: 'FeatureCollection',
			features: features
		};
	}

	// Listen for tree cache updates, update markers.
	public handleMount() {
		const unsunscribe = treeStore.subscribe((trees) => {
			this.updateGeoJSON(Object.values(trees));
		});

		return () => {
			unsunscribe();
		};
	}

	public handleClick(e: Event) {
		if (!e.features || e.features.length === 0) {
			return;
		}

		const treeId = e.features[0].properties.id;
		console.debug(`Tree ${treeId} clicked.`);
	}
}

export const mapState = new MapLibre();
