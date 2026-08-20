import { getPanoramasGeoJSON } from '$lib/api/panoramas';
import { mapBus } from '$lib/buses/mapBus';
import { showError } from '$lib/errors';
import { goto, routes } from '$lib/routes';
import { mapMarkerStore } from '$lib/stores/mapMarker.svelte';
import { mapPoiStore } from '$lib/stores/mapPoi.svelte';
import { mapZoom } from '$lib/stores/mapStore';
import type { IBounds } from '$lib/types';
import { Debouncer } from '$lib/utils/debounce';
import { LngLat } from 'maplibre-gl';
import { get } from 'svelte/store';

type Properties = {
	id: string;
	kind: 'image' | 'sequence';
	captured_at: number;
	compass_angle?: number;
	image_count?: number;
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

const extendBounds = ({ n, e, s, w }: IBounds): IBounds => {
	const dLat = n - s;
	const dLon = e - w;

	return {
		n: n + dLat,
		e: e + dLon,
		s: s - dLat,
		w: w - dLon
	};
};

export class PanoramicLayerState {
	data = $state.raw<Collection | undefined>(undefined);
	bounds = $state<IBounds | undefined>(undefined);
	fetchDebouncer = new Debouncer(200);

	private reload = () => {
		if (!this.bounds) {
			return;
		}

		const zoom = get(mapZoom);
		const { n, s, e, w } = extendBounds(this.bounds);

		this.fetchDebouncer.run(() => {
			getPanoramasGeoJSON(n, e, s, w, zoom >= 18, true)
				.then(({ status, data }) => {
					if (status === 200 && data) {
						const collection = data as unknown as Collection;
						console.debug(`[PanoramicLayer] Received ${collection.features.length} features.`);
						this.data = collection;
						mapPoiStore.panoramas = collection.features.map((f) => ({
							lat: f.geometry.coordinates[1],
							lon: f.geometry.coordinates[0],
							url: routes.panorama(f.properties.id)
						}));
					}
				})
				.catch((e) => {
					console.error('Error loading panoramas.', e);
					showError('Error loading panoramas, please try again.');
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
		mapMarkerStore.center = new LngLat(lng, lat);
		mapBus.emit('move', { lat, lng });

		console.debug(`[PanoramicLayer] Image ${id} clicked.`);

		await goto(routes.panorama(id));

		if (navigator.vibrate) {
			navigator.vibrate(50);
		}
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

export const panoramicLayerState = new PanoramicLayerState();
