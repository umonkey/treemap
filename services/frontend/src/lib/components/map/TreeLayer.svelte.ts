import { getGeoJSON } from '$lib/api/trees';
import { mapBus } from '$lib/buses/mapBus';
import { menuBus } from '$lib/buses/menuBus';
import { showError } from '$lib/errors';
import { goto, routes } from '$lib/routes';
import { mapPoiStore } from '$lib/stores/mapPoi.svelte';
import { searchStore } from '$lib/stores/searchStore';
import { Debouncer } from '$lib/utils/debounce';
import type { Map } from 'maplibre-gl';
import { getMapContext } from 'svelte-maplibre';
import { get } from 'svelte/store';
import { MapBouncer } from './MapBouncer';

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

class TreeLayerState {
	markers = $state.raw<Collection | undefined>(undefined);
	fetchDebouncer = new Debouncer(100);
	moveBouncer = new MapBouncer();
	private map: Map | undefined = undefined;

	public readonly crownRadiusSmall = 4;

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	public readonly crownRadiusLarge: any = [
		'interpolate',
		['exponential', 2],
		['zoom'],
		10,
		['*', ['get', 'crown'], 0.00428],
		22,
		['*', ['get', 'crown'], 17.534]
	];

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	public readonly trunkRadiusLarge: any = [
		'interpolate',
		['exponential', 2],
		['zoom'],
		10,
		['*', ['get', 'trunk'], 0.00428],
		22,
		['*', ['get', 'trunk'], 17.534]
	];

	private reload = (map: Map) => {
		const bounds = map.getBounds();
		const search = get(searchStore);
		const zoom = map.getZoom();

		const { n, e, s, w } = extendBounds({
			n: bounds.getNorth(),
			s: bounds.getSouth(),
			e: bounds.getEast(),
			w: bounds.getWest()
		});

		this.fetchDebouncer.run(() => {
			getGeoJSON(n, e, s, w, search, zoom)
				.then(({ status, data }) => {
					if (status === 200 && data) {
						const collection = data as unknown as Collection;
						console.debug(`[TreeLayer] Received ${collection.features.length} features.`);
						this.markers = collection;
						mapPoiStore.trees = collection.features.map((f) => ({
							lat: f.geometry.coordinates[1],
							lon: f.geometry.coordinates[0],
							url: routes.mapPreview(f.properties.id)
						}));
					}
				})
				.catch((e) => {
					console.error('Error loading trees.', e);
					showError('Error loading trees, please try again.');
				});
		});
	};

	private handleMove = (map: Map) => {
		const bounds = map.getBounds();

		if (!this.moveBouncer.changed(bounds)) {
			console.debug('TreeLayer reload triggered, but map not moved.');
			return;
		}

		this.reload(map);
	};

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	public handleClick = async (e: any) => {
		if (this.map && this.map.getZoom() < 15) {
			return;
		}

		if (!e.features || e.features.length === 0) {
			return;
		}

		const feature = e.features[0];
		const treeId = feature.properties.id;

		// Move the map immediately.  This makes us not wait until the tree
		// preview loads the data and displays it, but perform an animated move,
		// which makes the UI look more responsive.
		const [lng, lat] = feature.geometry.coordinates;
		mapBus.emit('move', { lat, lng });

		console.debug(`[TreeLayer] Tree ${treeId} clicked.`);

		await goto(routes.mapPreview(treeId));

		if (navigator.vibrate) {
			navigator.vibrate(50);
		}
	};

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	public handleContextMenu = (e: any) => {
		if (this.map && this.map.getZoom() < 15) {
			return;
		}

		if (!e.features || e.features.length === 0) {
			return;
		}

		const feature = e.features[0];
		const treeId = feature.properties.id;

		console.debug(`[TreeLayer] Tree ${treeId} context menu.`);

		if (navigator.vibrate) {
			navigator.vibrate(50);
		}

		menuBus.emit('show', treeId);
	};

	public onMount = () => {
		const map = getMapContext()?.map;

		if (!map) {
			console.warn('Map not available, cannot display trees.');
			return;
		}

		this.map = map;
		const handleMove = () => this.handleMove(map);
		const reload = () => this.reload(map);

		mapBus.on('reload', reload);

		if (map) {
			map.on('moveend', handleMove);
			map.on('zoomend', handleMove);
			reload();
		}

		return () => {
			this.map = undefined;
			mapBus.off('reload', reload);

			if (map) {
				map.off('moveend', handleMove);
				map.off('zoomend', handleMove);
			}
		};
	};
}

export const treeLayerState = new TreeLayerState();
