import { getWaterGeoJSON } from '$lib/api/water';
import { mapBus } from '$lib/buses/mapBus';
import { showError } from '$lib/errors';
import { extendBounds } from '$lib/map';
import { goto, routes } from '$lib/routes';
import { mapLayerStore } from '$lib/stores/mapLayerStore';
import type { IBounds } from '$lib/types';
import { Debouncer } from '$lib/utils/debounce';
import { get } from 'svelte/store';

type WaterFeature = {
	type: 'Feature';
	id: string;
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	geometry: any;
	properties: {
		id: string;
		status: string;
	};
};

type WaterCollection = {
	type: 'FeatureCollection';
	features: WaterFeature[];
};

export class WaterSourceLayerLogic {
	bounds = $state<IBounds | undefined>(undefined);
	markers = $state.raw<WaterCollection | undefined>(undefined);
	fetchDebouncer = new Debouncer(150);

	// Radius in pixels for a 50 meter disc on the ground, interpolated by zoom.
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	public readonly radius50m: any = [
		'interpolate',
		['exponential', 2],
		['zoom'],
		10,
		50 * 0.00428,
		22,
		50 * 17.534
	];

	private reload = () => {
		if (!this.bounds) {
			return;
		}

		if (get(mapLayerStore).water === false) {
			return;
		}

		const { n, e, s, w } = extendBounds(this.bounds);

		this.fetchDebouncer.run(() => {
			getWaterGeoJSON(n, e, s, w)
				.then(({ status, data }) => {
					if (!this.bounds) {
						return;
					}
					if (status === 200 && data) {
						console.debug(`[WaterSourceLayer] Received ${data.features.length} features.`);
						this.markers = data as unknown as WaterCollection;
					}
				})
				.catch((e) => {
					console.error('Error loading water sources.', e);
					showError('Error loading water sources, please try again.');
				});
		});
	};

	private handleBounds = (bounds: IBounds) => {
		this.bounds = bounds;
		this.reload();
	};

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	public handleClick = async (e: any) => {
		if (!e.features || e.features.length === 0) {
			return;
		}

		const feature = e.features[0];
		const id = feature.properties.id;

		const [lng, lat] = feature.geometry.coordinates;
		mapBus.emit('move', { lat, lng });

		console.debug(`[WaterSourceLayer] Water source ${id} clicked.`);

		await goto(routes.waterDetails(id));

		if (navigator.vibrate) {
			navigator.vibrate(50);
		}
	};

	public onMount = () => {
		mapBus.on('bounds', this.handleBounds);
		mapBus.on('reload', this.reload);

		return () => {
			this.bounds = undefined;
			this.markers = undefined;
			mapBus.off('bounds', this.handleBounds);
			mapBus.off('reload', this.reload);
		};
	};
}

export { WaterSourceLayerLogic as WaterSourceLayerState };
