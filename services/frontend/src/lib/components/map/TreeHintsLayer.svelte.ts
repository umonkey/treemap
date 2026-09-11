import { getPanoramasHints } from '$lib/api/panoramas';
import { mapBus } from '$lib/buses/mapBus';
import { panoBus } from '$lib/buses/panoBus';
import { showError } from '$lib/errors';
import { extendBounds } from '$lib/map';
import type { IBounds } from '$lib/types';
import { Debouncer } from '$lib/utils/debounce';
import { mapState } from './MapLibre.svelte.ts';

type Collection = {
	type: 'FeatureCollection';
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	features: any[];
};

export class TreeHintsLayerLogic {
	data = $state.raw<Collection | undefined>(undefined);
	bounds = $state<IBounds | undefined>(undefined);
	fetchDebouncer = new Debouncer(200);

	public reload = () => {
		if (!this.bounds || !mapState.treeHintsLayer) {
			return;
		}

		const { n, s, e, w } = extendBounds(this.bounds, 1);

		this.fetchDebouncer.run(() => {
			getPanoramasHints(n, e, s, w)
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

	private handleBounds = (bounds: IBounds) => {
		this.bounds = bounds;
		this.reload();
	};

	public onMount = () => {
		mapBus.on('bounds', this.handleBounds);
		panoBus.on('reload', this.reload);
		panoBus.on('reloadHints', this.reload);

		return () => {
			this.bounds = undefined;
			this.data = undefined;
			mapBus.off('bounds', this.handleBounds);
			panoBus.off('reload', this.reload);
			panoBus.off('reloadHints', this.reload);
		};
	};
}

export { TreeHintsLayerLogic as TreeHintsLayerState };
