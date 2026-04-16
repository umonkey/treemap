import { mapBus } from '$lib/buses/mapBus';
import { DEFAULT_MAP_CENTER } from '$lib/constants';
import { config } from '$lib/env';
import { mapLayerStore } from '$lib/stores/mapLayerStore';
import { mapStore } from '$lib/stores/mapStore';
import type { ILatLng } from '$lib/types';
import { Debouncer } from '$lib/utils/debounce';
import {
	type LngLat,
	LngLat as LngLat2,
	type LngLatBounds,
	LngLatBounds as LngLatBounds2,
	type Map,
	type StyleSpecification
} from 'maplibre-gl';
import { get } from 'svelte/store';
import { MapBouncer } from './MapBouncer';

const BASIC_LAYER = `https://api.maptiler.com/maps/openstreetmap/style.json?key=${config.mapTilerKey}`;
const LIGHT_LAYER = 'https://basemaps.cartocdn.com/gl/positron-gl-style/style.json';
const DRONE_LAYER = 'https://treemap-tiles.fra1.cdn.digitaloceanspaces.com/{z}/{x}/{y}.png';

const BING_LAYER: StyleSpecification = {
	version: 8,
	sources: {
		'bing-satellite': {
			type: 'raster',
			tiles: [
				'https://ecn.t0.tiles.virtualearth.net/tiles/a{quadkey}.jpeg?g=1',
				'https://ecn.t1.tiles.virtualearth.net/tiles/a{quadkey}.jpeg?g=1',
				'https://ecn.t2.tiles.virtualearth.net/tiles/a{quadkey}.jpeg?g=1',
				'https://ecn.t3.tiles.virtualearth.net/tiles/a{quadkey}.jpeg?g=1'
			],
			tileSize: 256,
			attribution: '© Microsoft Bing'
		}
	},
	layers: [{ id: 'bing-satellite', type: 'raster', source: 'bing-satellite' }]
};

const GOOGLE_LAYER: StyleSpecification = {
	version: 8,
	sources: {
		'google-satellite': {
			type: 'raster',
			tiles: ['https://mt1.google.com/vt/lyrs=y&x={x}&y={y}&z={z}'],
			tileSize: 256,
			attribution: '© Google'
		}
	},
	layers: [{ id: 'google-satellite', type: 'raster', source: 'google-satellite' }]
};

class MapLibre {
	map = $state.raw<Map>();
	layer = $state<string | StyleSpecification>(BASIC_LAYER);
	droneLayer = $state<string | undefined>(undefined);

	zoom = $state<number>(13);
	center = $state<ILatLng>(DEFAULT_MAP_CENTER);
	bounds = $state<LngLatBounds>();
	marker = $state<LngLat>();

	mapBouncer = new MapBouncer();

	onMove: (ll: ILatLng) => void = () => {};

	storeDebouncer = new Debouncer(500);

	public constructor() {
		this.zoom = get(mapStore)?.zoom ?? 13;
		this.center = get(mapStore)?.center ?? DEFAULT_MAP_CENTER;

		console.debug(`Read map center from mapStore: ${this.center.lat},${this.center.lng}`);
	}

	public handleLoad = () => {
		if (this.map) {
			this.bounds = this.map.getBounds();
			console.debug('MapLibre load fired.');
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

		if (!this.mapBouncer.changed(this.bounds)) {
			return;
		}

		this.updateStore(this.bounds);

		mapBus.emit('center', this.center);

		if (this.onMove) {
			this.onMove(this.center);
		}
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

	private handleMoveRequest = (ll: ILatLng) => {
		console.debug(`Handling request to move the map to ${ll.lat},${ll.lng}`);
		this.center = ll;
	};

	public onMount = () => {
		mapBus.on('pin', this.handlePinChange);
		mapBus.on('fit', this.handleFit);
		mapBus.on('move', this.handleMoveRequest);

		const unsub = mapLayerStore.subscribe(() => {
			this.updateLayers();
		});

		console.debug(`MapLibre initialized with center in ${this.center.lat},${this.center.lng}`);

		this.updateLayers();

		return () => {
			mapBus.off('pin', this.handlePinChange);
			mapBus.off('fit', this.handleFit);
			mapBus.off('move', this.handleMoveRequest);
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
		} else if (base === 'bing') {
			this.layer = BING_LAYER;
		} else if (base === 'google') {
			this.layer = GOOGLE_LAYER;
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
