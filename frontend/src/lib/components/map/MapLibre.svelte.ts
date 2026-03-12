import { apiClient } from '$lib/api';
import { mapStore } from '$lib/stores/mapStore';
import type { LngLatBounds } from 'maplibre-gl';
import { get } from 'svelte/store';

type Properties = {
	id: string;
	state: string;
	type: string;
	crown: number;
};

type Feature = {
	type: 'Feature';
	id: string;
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	geometry: any;
	properties: Properties;
};

type Collection = {
	type: 'FeatureCollection';
	features: Feature[];
};

class MapLibre {
	markers = $state.raw<Collection | undefined>(undefined);
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
		const n = bounds.getNorth();
		const s = bounds.getSouth();
		const e = bounds.getEast();
		const w = bounds.getWest();

		const dLat = n - s;
		const dLon = e - w;

		this.reloadTrees(n + dLat / 2, e + dLon / 2, s - dLat / 2, w - dLon / 2);
		this.updateStore(bounds);
	};

	public handleMoveStart = () => {
		clearTimeout(this.fetchTimeout);
	};

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	public handleClick = (e: any) => {
		if (!e.features || e.features.length === 0) {
			return;
		}

		const treeId = e.features[0].properties.id;
		console.debug(`Tree ${treeId} clicked.`);
	};

	private reloadTrees(n: number, e: number, s: number, w: number) {
		clearTimeout(this.fetchTimeout);

		this.fetchTimeout = setTimeout(() => {
			console.debug('Requesting markers...');

			apiClient.getGeoJSON(n, e, s, w).then(({ status, data }) => {
				if (status === 200 && data) {
					const collection = data as unknown as Collection;
					console.debug(`Received ${collection.features.length} features.`);
					this.markers = collection;
				}
			});
		}, 1000);
	}
}

export const mapState = new MapLibre();
