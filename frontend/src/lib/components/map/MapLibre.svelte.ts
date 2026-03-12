import { apiClient } from '$lib/api';
import { mapStore } from '$lib/stores/mapStore';
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
					console.debug(`Received ${data.features.length} features.`);
					this.markers = data.features;
				}
			});
		}, 1000);
	}
}

export const mapState = new MapLibre();
