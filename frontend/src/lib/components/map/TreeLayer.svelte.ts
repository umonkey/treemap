import { apiClient } from '$lib/api';
import { mapBus } from '$lib/buses';
import { showError } from '$lib/errors';
import { searchStore } from '$lib/stores/searchStore';
import { Debouncer } from '$lib/utils/debounce';
import { MapBouncer } from './MapBouncer';
import type { Map } from 'maplibre-gl';
import { getMapContext } from 'svelte-maplibre';
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

export class TreeLayerState {
	markers = $state.raw<Collection | undefined>(undefined);
	fetchDebouncer = new Debouncer(100);
	moveBouncer = new MapBouncer();

	public reloadTrees = (map: Map) => {
		const bounds = map.getBounds();
		const search = get(searchStore);

		if (!this.moveBouncer.changed(bounds)) {
			console.debug('TreeLayer reload triggered, but map not moved.');
			return;
		}

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
						console.debug(`[TreeLayer] Received ${collection.features.length} features.`);
						this.markers = collection;
					}
				})
				.catch((e) => {
					console.error('Error loading trees.', e);
					showError('Error loading trees, please try again.');
				});
		});
	};

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	public handleClick = (e: any) => {
		if (!e.features || e.features.length === 0) {
			return;
		}

		const feature = e.features[0];
		const treeId = feature.properties.id;
		console.debug(`[TreeLayer] Tree ${treeId} clicked.`);

		mapBus.emit('select', treeId);

		if (navigator.vibrate) {
			navigator.vibrate(50);
		}
	};

	public onMount = () => {
		const map = getMapContext()?.map;

		if (!map) {
			console.error('Map not available, cannot display trees.');
			return;
		}

		const reload = () => this.reloadTrees(map);

		mapBus.on('reload', reload);

		if (map) {
			map.on('moveend', reload);
			map.on('zoomend', reload);
			reload();
		}

		const unsubSearch = searchStore.subscribe(reload);

		return () => {
			mapBus.off('reload', reload);

			if (map) {
				map.off('moveend', reload);
				map.off('zoomend', reload);
			}

			unsubSearch();
		};
	};
}

export const treeLayerState = new TreeLayerState();
