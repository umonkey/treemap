import { apiClient } from '$lib/api';
import { mapBus } from '$lib/buses';
import { goto, routes } from '$lib/routes';
import { mapStore } from '$lib/stores/mapStore';
import type { ILatLng } from '$lib/types';
import { Debouncer } from '$lib/utils/debounce';
import { type LngLat, type LngLatBounds, LngLat as LngLat2 } from 'maplibre-gl';
import { get } from 'svelte/store';

type Properties = {
	id: string;
	state: string;
	type: string;
	crown: number;
	trunk: number;
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
	center = $state<ILatLng | undefined>(undefined);
	marker = $state<LngLat>();

	fetchDebouncer = new Debouncer(100);
	storeDebouncer = new Debouncer(500);

	public constructor() {
		this.zoom = get(mapStore)?.zoom ?? 13;
		this.center = get(mapStore)?.center;
	}

	private updateStore(bounds?: LngLatBounds) {
		this.storeDebouncer.run(() => {
			mapStore.update((s) => {
				const newState = { ...s, zoom: this.zoom };

				if (bounds) {
					const center = bounds.getCenter();
					newState.center = { lat: center.lat, lng: center.lng };
				}

				return newState;
			});
		});
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
		// pass
	};

	// Handles a click to a tree.
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	public handleClick = (e: any) => {
		if (!e.features || e.features.length === 0) {
			return;
		}

		const feature = e.features[0];
		const treeId = feature.properties.id;
		console.debug(`Tree ${treeId} clicked.`);

		mapBus.emit('select', treeId);

		if (navigator.vibrate) {
			navigator.vibrate(50);
		}

		if (feature.geometry?.type === 'Point') {
			const [lng, lat] = feature.geometry.coordinates;
			this.center = { lat, lng };
			this.marker = new LngLat2(lng, lat);
		}
	};

	public handleAddTree = (ll: LngLat) => {
		const { lat, lng } = ll;
		goto(routes.treeAdd(lat, lng));
	};

	public handleAddRow = (start: LngLat, end: LngLat) => {
		goto(routes.addRow(start, end));
	};

	private reloadTrees(n: number, e: number, s: number, w: number) {
		this.fetchDebouncer.run(() => {
			apiClient.getGeoJSON(n, e, s, w).then(({ status, data }) => {
				if (status === 200 && data) {
					const collection = data as unknown as Collection;
					console.debug(`Received ${collection.features.length} features.`);
					this.markers = collection;
				}
			});
		});
	}
}

export const mapState = new MapLibre();
