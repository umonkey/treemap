import { mapBus } from '$lib/buses/mapBus';
import { DEFAULT_MAP_CENTER } from '$lib/constants';
import { config } from '$lib/env';
import { locale } from '$lib/locale';
import { mapLayerStore } from '$lib/stores/mapLayerStore';
import { mapStore } from '$lib/stores/mapStore';
import type { ILatLng } from '$lib/types';
import { Debouncer } from '$lib/utils/debounce';
import { type LngLatBounds, type Map, type StyleSpecification } from 'maplibre-gl';
import { get } from 'svelte/store';
import { MapBouncer } from './MapBouncer';

const BASIC_LAYER = `https://api.maptiler.com/maps/openstreetmap/style.json?key=${config.mapTilerKey}&language=${locale.lang}`;
const LIGHT_LAYER = `https://api.maptiler.com/maps/base-v4-light/style.json?key=${config.mapTilerKey}&language=${locale.lang}`;
const DRONE_LAYER = 'https://treemap-tiles.fra1.cdn.digitaloceanspaces.com/{z}/{x}/{y}.png';

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
	layer = $state<string | StyleSpecification>(LIGHT_LAYER);
	droneLayer = $state<string | undefined>(undefined);
	panoramasLayer = $state<boolean>(false);
	treeHintsLayer = $state<boolean>(false);

	zoom = $state<number>(13);
	bearing = $state<number>(0);
	center = $state<ILatLng>(DEFAULT_MAP_CENTER);
	bounds = $state<LngLatBounds>();
	bottomPadding = $state<number>(0);

	private paddingOwner: symbol | null = null;

	mapBouncer = new MapBouncer();

	onMove: (ll: ILatLng) => void = () => {};

	storeDebouncer = new Debouncer(500);

	public constructor() {
		this.zoom = get(mapStore)?.zoom ?? 13;
		this.bearing = get(mapStore)?.bearing ?? 0;
		this.center = get(mapStore)?.center ?? DEFAULT_MAP_CENTER;

		console.debug(`Read map center from mapStore: ${this.center.lat},${this.center.lng}`);
	}

	public handleLoad = () => {
		if (this.map) {
			this.bounds = this.map.getBounds();
			this.applyPadding();
			console.debug('MapLibre load fired.');
			this.handleMoveEnd();
		}
	};

	public setBottomPadding = (px: number, owner: symbol) => {
		const value = Math.max(0, Math.round(px));
		this.paddingOwner = owner;
		this.bottomPadding = value;
		this.applyPadding();
	};

	public clearBottomPadding = (owner: symbol) => {
		if (this.paddingOwner !== owner) return;
		this.paddingOwner = null;
		this.bottomPadding = 0;
		this.applyPadding();
	};

	private applyPadding = () => {
		this.map?.setPadding({ bottom: this.bottomPadding });
	};

	private updateStore = (bounds?: LngLatBounds) => {
		this.storeDebouncer.run(() => {
			mapStore.update((s) => {
				const newState = { ...s, zoom: this.zoom, bearing: this.bearing };

				if (bounds) {
					const center = this.map?.getCenter();
					if (center) {
						newState.center = { lat: center.lat, lng: center.lng };
					}
				}

				return newState;
			});
		});
	};

	public handleZoom = () => {
		this.updateStore();
	};

	public handleRotate = () => {
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

		mapBus.emit('bounds', {
			n: this.bounds.getNorth(),
			e: this.bounds.getEast(),
			s: this.bounds.getSouth(),
			w: this.bounds.getWest(),
			zoom: this.zoom
		});

		if (this.onMove) {
			this.onMove(this.center);
		}
	};

	public onMount = () => {
		const unsub = mapLayerStore.subscribe(() => {
			this.updateLayers();
		});

		console.debug(`MapLibre initialized with center in ${this.center.lat},${this.center.lng}`);

		this.updateLayers();

		return () => {
			unsub();
		};
	};

	private updateLayers = () => {
		console.debug('Updating layers...');

		const base = get(mapLayerStore).base;

		if (base === 'basic') {
			this.layer = BASIC_LAYER;
		} else if (base === 'google') {
			this.layer = GOOGLE_LAYER;
		} else {
			this.layer = LIGHT_LAYER;
		}

		if (get(mapLayerStore).drone) {
			this.droneLayer = DRONE_LAYER;
		} else {
			this.droneLayer = undefined;
		}

		this.panoramasLayer = get(mapLayerStore).panoramas === true;
		this.treeHintsLayer = get(mapLayerStore).treeHints === true;
	};
}

export const mapState = new MapLibre();
