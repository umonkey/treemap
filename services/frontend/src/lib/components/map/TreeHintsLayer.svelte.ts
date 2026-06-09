import { getMapillaryHints } from '$lib/api/mapillary';
import { showError } from '$lib/errors';
import { Debouncer } from '$lib/utils/debounce';
import { type Map } from 'maplibre-gl';
import { getMapContext } from 'svelte-maplibre';
import { MapBouncer } from './MapBouncer';

type Collection = {
	type: 'FeatureCollection';
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	features: any[];
};

const extendBounds = ({ n, e, s, w }: { n: number; e: number; s: number; w: number }) => {
	const dLat = n - s;
	const dLon = e - w;

	return {
		n: n + dLat,
		e: e + dLon,
		s: s - dLat,
		w: w - dLon
	};
};

class TreeHintsLayerState {
	data = $state.raw<Collection | undefined>(undefined);
	fetchDebouncer = new Debouncer(200);
	moveBouncer = new MapBouncer();

	private reload = (map: Map) => {
		const bounds = map.getBounds();

		const { n, s, e, w } = extendBounds({
			n: bounds.getNorth(),
			s: bounds.getSouth(),
			e: bounds.getEast(),
			w: bounds.getWest()
		});

		this.fetchDebouncer.run(() => {
			getMapillaryHints(n, e, s, w)
				.then(({ status, data }) => {
					if (status === 200 && data) {
						const collection = data as unknown as Collection;
						console.debug(`[TreeHintsLayer] Received ${collection.features.length} features.`);
						this.data = collection;
					}
				})
				.catch((e) => {
					console.error('Error loading tree hints.', e);
					showError('Error loading tree hints, please try again.');
				});
		});
	};

	private handleMove = (map: Map) => {
		const bounds = map.getBounds();

		if (!this.moveBouncer.changed(bounds)) {
			console.debug('TreeHintsLayer reload triggered, but map not moved.');
			return;
		}

		this.reload(map);
	};

	public onMount = () => {
		const map = getMapContext()?.map;

		if (!map) {
			console.warn('Map not available, cannot display tree hints.');
			return;
		}

		const handleMove = () => this.handleMove(map);
		const reload = () => this.reload(map);

		map.on('moveend', handleMove);
		map.on('zoomend', handleMove);
		reload();

		return () => {
			map.off('moveend', handleMove);
			map.off('zoomend', handleMove);
		};
	};
}

export const treeHintsLayerState = new TreeHintsLayerState();
