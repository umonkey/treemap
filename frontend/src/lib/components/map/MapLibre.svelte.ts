import { apiClient } from '$lib/api';
import { mapStore } from '$lib/stores/mapStore';
import { addTrees, treeStore } from '$lib/stores/treeStore';
import { type ITree } from '$lib/types';
import circle from '@turf/circle';
import type { LngLatBounds } from 'maplibre-gl';
import { get } from 'svelte/store';

type Properties = {
	id: string;
	state: string;
	type: string;
};

type Feature = {
	type: 'Feature';
	id: string;
	geometry: any;
	properties: Properties;
};

type Collection = {
	type: 'FeatureCollection';
	features: Feature[];
};

class MapLibre {
	markers = $state<Collection | undefined>(undefined);
	zoom = $state<number>(13);

	fetchTimeout: ReturnType<typeof setTimeout> | undefined = undefined;
	storeTimeout: ReturnType<typeof setTimeout> | undefined = undefined;

	public constructor() {
		this.zoom = get(mapStore)?.zoom ?? 13;
	}

	private updateStore(bounds?: LngLatBounds) {
		clearTimeout(this.storeTimeout);
		this.storeTimeout = setTimeout(() => {
			mapStore.update((s) => {
				const newState = { ...s, zoom: this.zoom };
				if (bounds) {
					const center = bounds.getCenter();
					newState.center = { lat: center.lat, lng: center.lng };
				}
				return newState;
			});
		}, 500);
	}

	public handleZoom = () => {
		this.updateStore();
	};

	public handleMoveEnd = (bounds: LngLatBounds) => {
		this.reloadTrees(bounds.getNorth(), bounds.getEast(), bounds.getSouth(), bounds.getWest());
		this.updateStore(bounds);
	};

	public handleMoveStart = () => {
		clearTimeout(this.fetchTimeout);
	};

	public handleMount = () => {
		const unsunscribe = treeStore.subscribe((trees) => {
			this.updateGeoJSON(Object.values(trees));
		});

		return () => {
			unsunscribe();
		};
	};

	public handleClick = (e: any) => {
		if (!e.features || e.features.length === 0) {
			return;
		}

		const treeId = e.features[0].properties.id;
		console.debug(`Tree ${treeId} clicked.`);
	};

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
			} as Feature);

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
			} as Feature);

			return features;
		});

		return {
			type: 'FeatureCollection',
			features: features
		};
	}
}

export const mapState = new MapLibre();
