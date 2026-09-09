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
	enabled = $state<boolean>(true);
	fetchDebouncer = new Debouncer(150);

	constructor() {
		this.enabled = get(mapLayerStore).water !== false;
	}

	// Radius in pixels for a 50 meter disc on the ground, interpolated by zoom.
	//
	// MapLibre renders `circle-radius` in pixels. To get a fixed ground size we
	// convert meters to pixels using the pixel-per-meter scale at Yerevan's
	// latitude (40.181389N) for MapLibre's 512px tiles:
	//   px/m(z) = 512 * 2^z / (2 * PI * 6371008.8 * cos(lat))
	// At zoom 10: 0.017142 px/m, at zoom 22: 70.216 px/m (ratio 2^12).
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	public readonly radius50m: any = [
		'interpolate',
		['exponential', 2],
		['zoom'],
		10,
		50 * 0.017142,
		22,
		50 * 70.216
	];

	private reload = () => {
		if (!this.bounds) {
			return;
		}

		if (!this.enabled) {
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

		const unsub = mapLayerStore.subscribe((layers) => {
			const wasEnabled = this.enabled;
			this.enabled = layers.water !== false;
			if (!wasEnabled && this.enabled && !this.markers) {
				this.reload();
			}
		});

		return () => {
			this.bounds = undefined;
			this.markers = undefined;
			mapBus.off('bounds', this.handleBounds);
			mapBus.off('reload', this.reload);
			unsub();
		};
	};
}

export { WaterSourceLayerLogic as WaterSourceLayerState };
