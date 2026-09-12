import { getPanoramasGeoJSON } from '$lib/api/panoramas';
import { mapBus } from '$lib/buses/mapBus';
import { showError } from '$lib/errors';
import { extendBounds } from '$lib/map';
import { goto, routes } from '$lib/routes';
import { mapPoiStore } from '$lib/stores/mapPoi.svelte';
import { mapZoom } from '$lib/stores/mapStore';
import type { IBounds } from '$lib/types';
import { Debouncer } from '$lib/utils/debounce';
import { get } from 'svelte/store';
import { mapState } from './MapLibre.svelte.ts';

type Properties = {
	id: string;
	kind: 'image' | 'sequence';
	captured_at: number;
	compass_angle?: number;
	image_count?: number;
};

type PanoramaGeometry =
	{ type: 'Point'; coordinates: number[] } | { type: 'MultiLineString'; coordinates: number[][][] };

type Feature = {
	type: 'Feature';
	id: string;
	geometry: PanoramaGeometry;
	properties: Properties;
};

type Collection = {
	type: 'FeatureCollection';
	features: Feature[];
};

/**
 * Normalize sequence coordinates to a MultiLineString sections array.
 *
 * Legacy API responses return depth-2 coordinates (a flat LineString), while
 * the current backend returns depth-3 coordinates (an array of sections).
 */
export function fixSequenceFormat(coordinates: number[][] | number[][][]): number[][][] {
	if (coordinates.length === 0) return [];

	return typeof coordinates[0][0] === 'number'
		? [coordinates as number[][]]
		: (coordinates as number[][][]);
}

export class PanoramicLayerLogic {
	data = $state.raw<Collection | undefined>(undefined);
	bounds = $state<IBounds | undefined>(undefined);
	fetchDebouncer = new Debouncer(200);

	public reload = () => {
		if (!this.bounds || !mapState.panoramasLayer) {
			return;
		}

		const zoom = get(mapZoom);
		const { n, s, e, w } = extendBounds(this.bounds, 1);

		this.fetchDebouncer.run(() => {
			getPanoramasGeoJSON(n, e, s, w, zoom >= 18, true)
				.then(({ status, data }) => {
					if (status === 200 && data) {
						const collection = data as unknown as Collection;

						collection.features.forEach((feature) => {
							if (feature.properties?.kind === 'sequence') {
								feature.geometry = {
									type: 'MultiLineString',
									coordinates: fixSequenceFormat(
										feature.geometry.coordinates as number[][] | number[][][]
									)
								};
							}
						});

						console.debug(`[PanoramicLayer] Received ${collection.features.length} features.`);
						this.data = collection;
						mapPoiStore.panoramas = collection.features
							.filter((f) => f.properties?.kind === 'image' && f.geometry?.type === 'Point')
							.map((f) => {
								const [lon, lat] = f.geometry.coordinates as [number, number];

								return {
									lat,
									lon,
									url: routes.panorama(f.properties.id)
								};
							});
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
		mapBus.emit('pin', { lat, lng });
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
			mapPoiStore.panoramas = [];
		};
	};
}

export { PanoramicLayerLogic as PanoramicLayerState };
