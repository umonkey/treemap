import { getGeoJSON } from '$lib/api/trees';
import { mapBus } from '$lib/buses/mapBus';
import { menuBus } from '$lib/buses/menuBus';
import { showError } from '$lib/errors';
import { extendBounds } from '$lib/map';
import { goto, routes } from '$lib/routes';
import { mapPoiStore } from '$lib/stores/mapPoi.svelte';
import { mapZoom } from '$lib/stores/mapStore';
import { searchStore } from '$lib/stores/searchStore';
import type { IBounds } from '$lib/types';
import { Debouncer } from '$lib/utils/debounce';
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

export class TreeLayerState {
	bounds = $state<IBounds | undefined>(undefined);
	markers = $state.raw<Collection | undefined>(undefined);
	fetchDebouncer = new Debouncer(100);

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

	private reload = () => {
		if (!this.bounds) {
			return;
		}

		const search = get(searchStore);
		const zoom = get(mapZoom);

		const { n, e, s, w } = extendBounds(this.bounds);

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

	private handleBounds = (bounds: IBounds) => {
		this.bounds = bounds;
		this.reload();
	};

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	public handleClick = async (e: any) => {
		if (get(mapZoom) < 15) {
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
		if (get(mapZoom) < 15) {
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
		mapBus.on('bounds', this.handleBounds);
		mapBus.on('reload', this.reload);

		return () => {
			this.bounds = undefined;
			mapBus.off('bounds', this.handleBounds);
			mapBus.off('reload', this.reload);
		};
	};
}

export const treeLayerState = new TreeLayerState();
