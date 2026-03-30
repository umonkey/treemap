import { apiClient } from '$lib/api';
import { mapBus } from '$lib/buses';
import { DEFAULT_MAP_CENTER } from '$lib/constants';
import { config } from '$lib/env';
import { showError } from '$lib/errors';
import { mapLayerStore } from '$lib/stores/mapLayerStore';
import { mapStore } from '$lib/stores/mapStore';
import { searchStore } from '$lib/stores/searchStore';
import type { ILatLng } from '$lib/types';
import { Debouncer } from '$lib/utils/debounce';
import {
	type LngLat,
	LngLat as LngLat2,
	type LngLatBounds,
	LngLatBounds as LngLatBounds2,
	type Map
} from 'maplibre-gl';
import { get } from 'svelte/store';

const BASIC_LAYER = `https://api.maptiler.com/maps/openstreetmap/style.json?key=${config.mapTilerKey}`;
const LIGHT_LAYER = 'https://basemaps.cartocdn.com/gl/positron-gl-style/style.json';
const DARK_LAYER = 'https://basemaps.cartocdn.com/gl/dark-matter-gl-style/style.json';
const DRONE_LAYER = 'https://treemap-tiles.fra1.cdn.digitaloceanspaces.com/{z}/{x}/{y}.png';

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

const extendBounds = ({ n, e, s, w }: { n: number; e: number; s: number; w: number }) => {
	const dLat = n - s;
	const dLon = e - w;

	return {
		n: n + dLat / 2,
		e: e + dLon / 2,
		s: s - dLat / 2,
		w: w - dLon / 2
	};
};

class MapLibre {
	map = $state.raw<Map>();
	layer = $state<string>(BASIC_LAYER);
	droneLayer = $state<string | undefined>(undefined);

	markers = $state.raw<Collection | undefined>(undefined);
	zoom = $state<number>(13);
	center = $state<ILatLng>(DEFAULT_MAP_CENTER);
	bounds = $state<LngLatBounds>();
	marker = $state<LngLat>();

	// Last moved bounds, used to prevent cyclic updates.
	lastBounds = $state<string>();

	onMove: (ll: ILatLng) => void = () => {};

	fetchDebouncer = new Debouncer(100);
	storeDebouncer = new Debouncer(500);

	public constructor() {
		this.zoom = get(mapStore)?.zoom ?? 13;
		this.center = get(mapStore)?.center ?? DEFAULT_MAP_CENTER;
	}

	public handleLoad = () => {
		if (this.map) {
			this.bounds = this.map.getBounds();
			console.debug('MapLibre load fired, loading the markers.');
			this.handleMoveEnd();
		}
	};

	public handleFit = ({ start, end }: { start: ILatLng; end: ILatLng }) => {
		if (this.map) {
			const bounds = new LngLatBounds2();
			bounds.extend([start.lng, start.lat]);
			bounds.extend([end.lng, end.lat]);
			this.map.fitBounds(bounds, { padding: 50 });
		}
	};

	private updateStore = (bounds?: LngLatBounds) => {
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
	};

	public handleZoom = () => {
		this.updateStore();
	};

	public handleMoveEnd = () => {
		if (!this.bounds) {
			console.debug('Bounds not set, ignoring MapLibre move.');
			return;
		}

		if (!this.boundsChanged(this.bounds)) {
			return;
		}

		this.reloadTrees();
		this.updateStore(this.bounds);

		mapBus.emit('center', this.center);

		if (this.onMove) {
			this.onMove(this.center);
		}
	};

	private boundsChanged = (bounds: LngLatBounds): boolean => {
		const hash = `W=${bounds.getWest().toFixed(5)} S=${bounds.getSouth().toFixed(5)} E=${bounds.getEast().toFixed(5)} N=${bounds.getNorth().toFixed(5)}`;

		if (hash !== this.lastBounds) {
			this.lastBounds = hash;
			console.debug(`MapLibre moved, new bounds: ${hash}`);
			return true;
		}

		console.debug('Oops, duplicate MapLibre move.');

		return false;
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
	};

	private reloadTrees = () => {
		const bounds = this.bounds;

		if (!bounds) {
			return;
		}

		const search = get(searchStore);

		const { n, e, s, w } = extendBounds({
			n: bounds.getNorth(),
			s: bounds.getSouth(),
			e: bounds.getEast(),
			w: bounds.getWest()
		});

		this.fetchDebouncer.run(() => {
			apiClient
				.getGeoJSON(n, e, s, w, search)
				.then(({ status, data }) => {
					if (status === 200 && data) {
						const collection = data as unknown as Collection;
						console.debug(`Received ${collection.features.length} features.`);
						this.markers = collection;
					}
				})
				.catch((e) => {
					console.error('Error loading trees.', e);
					showError('Error loading trees, please try again.');
				});
		});
	};

	// This is triggered by the MapPreview element via mapBus, to tell us
	// that the user clicked another tree, or closed the preview.
	public handlePinChange = (ll: ILatLng | undefined) => {
		if (ll) {
			this.marker = this.ll(ll);
			this.center = ll;
		} else {
			this.marker = undefined;
		}
	};

	public onMount = () => {
		mapBus.on('pin', this.handlePinChange);
		mapBus.on('fit', this.handleFit);
		mapBus.on('reload', this.reloadTrees);

		const unsub = mapLayerStore.subscribe(() => {
			this.updateLayers();
		});

		this.updateLayers();

		return () => {
			mapBus.off('pin', this.handlePinChange);
			mapBus.off('fit', this.handleFit);
			mapBus.off('reload', this.reloadTrees);
			unsub();
		};
	};

	private ll = (ll: ILatLng): LngLat => {
		return new LngLat2(ll.lng, ll.lat);
	};

	private updateLayers = () => {
		console.debug('Updating layers...');

		const base = get(mapLayerStore).base;

		if (base === 'light') {
			this.layer = LIGHT_LAYER;
		} else if (base === 'dark') {
			this.layer = DARK_LAYER;
		} else {
			this.layer = BASIC_LAYER;
		}

		if (get(mapLayerStore).drone) {
			this.droneLayer = DRONE_LAYER;
		} else {
			this.droneLayer = undefined;
		}
	};
}

export const mapState = new MapLibre();
