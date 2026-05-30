import { mapBus } from '$lib/buses/mapBus';
import { DEFAULT_MAP_CENTER } from '$lib/constants';
import { config } from '$lib/env';
import { mapLayerStore } from '$lib/stores/mapLayerStore';
import { mapMode } from '$lib/stores/mapMode';
import { mapStore } from '$lib/stores/mapStore';
import { goto, routes } from '$lib/routes';
import type { ILatLng } from '$lib/types';
import { Debouncer } from '$lib/utils/debounce';
import { getDistance } from '$lib/utils/geo';
import {
	type LngLatBounds,
	LngLatBounds as LngLatBounds2,
	type Map,
	type MapLibreEvent,
	type StyleSpecification
} from 'maplibre-gl';
import { get } from 'svelte/store';
import { MapBouncer } from './MapBouncer';
import { treeLayerState } from './TreeLayer.svelte.ts';

const BASIC_LAYER = `https://api.maptiler.com/maps/openstreetmap/style.json?key=${config.mapTilerKey}`;
const LIGHT_LAYER = 'https://basemaps.cartocdn.com/gl/positron-gl-style/style.json';
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
	layer = $state<string | StyleSpecification>(BASIC_LAYER);
	droneLayer = $state<string | undefined>(undefined);
	alertsLayer = $state<boolean>(true);
	panoramasLayer = $state<boolean>(false);

	hasMoved = false;
	moving = $state(false);
	zoom = $state<number>(13);
	bearing = $state<number>(0);
	center = $state<ILatLng>(DEFAULT_MAP_CENTER);
	bounds = $state<LngLatBounds>();

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
			console.debug('MapLibre load fired.');
			this.handleMoveEnd();
		}
	};

	public handleMoveStart = (e?: MapLibreEvent) => {
		if (e?.originalEvent) {
			this.moving = true;
			this.hasMoved = true;
		}
	};

	public handleFit = ({ start, end }: { start: ILatLng; end: ILatLng }) => {
		this.hasMoved = true;
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
				const newState = { ...s, zoom: this.zoom, bearing: this.bearing };

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

	public handleRotate = () => {
		this.updateStore();
	};

	public handleMoveEnd = (e?: MapLibreEvent) => {
		this.moving = false;

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

		const mode = get(mapMode);
		const isUserAction = !!e?.originalEvent;

		if (isUserAction && (mode === undefined || mode === 'preview') && this.zoom > 18) {
			const collection = treeLayerState.markers;
			if (collection && collection.features.length) {
				let minDistance = Infinity;
				let nearestFeature = null;

				for (const feature of collection.features) {
					const [lng, lat] = feature.geometry.coordinates;
					const dist = getDistance(this.center, { lat, lng });
					if (dist < minDistance) {
						minDistance = dist;
						nearestFeature = feature;
					}
				}

				if (nearestFeature && minDistance <= 5) {
					const nearestId = nearestFeature.properties.id;
					const [lng, lat] = nearestFeature.geometry.coordinates;
					mapBus.emit('move', { lat, lng });
					console.debug(`Snapping to nearest tree ${nearestId} (${minDistance.toFixed(1)}m)`);
					goto(routes.mapPreview(nearestId));
				}
			}
		}
	};

	private handleMoveRequest = (ll: ILatLng) => {
		console.debug(`Handling request to move the map to ${ll.lat},${ll.lng}`);
		this.hasMoved = true;
		this.center = ll;
	};

	private handleMapOnceRequest = (ll: ILatLng) => {
		if (this.hasMoved) {
			console.debug(
				`Ignoring map-once request to ${ll.lat},${ll.lng} because the map has already moved.`
			);
			return;
		}

		this.handleMoveRequest(ll);
	};

	public onMount = () => {
		mapBus.on('fit', this.handleFit);
		mapBus.on('move', this.handleMoveRequest);
		mapBus.on('map-once', this.handleMapOnceRequest);

		const unsub = mapLayerStore.subscribe(() => {
			this.updateLayers();
		});

		console.debug(`MapLibre initialized with center in ${this.center.lat},${this.center.lng}`);

		this.updateLayers();

		return () => {
			mapBus.off('fit', this.handleFit);
			mapBus.off('move', this.handleMoveRequest);
			mapBus.off('map-once', this.handleMapOnceRequest);
			unsub();
		};
	};

	private updateLayers = () => {
		console.debug('Updating layers...');

		const base = get(mapLayerStore).base;

		if (base === 'light') {
			this.layer = LIGHT_LAYER;
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

		this.alertsLayer = get(mapLayerStore).alerts !== false;
		this.panoramasLayer = get(mapLayerStore).panoramas === true;
	};
}

export const mapState = new MapLibre();
