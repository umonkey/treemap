import { getMapillaryGeoJSON } from '$lib/api/mapillary';
import { mapBus } from '$lib/buses/mapBus';
import { showError } from '$lib/errors';
import { goto, routes } from '$lib/routes';
import { mapPoiStore } from '$lib/stores/mapPoi.svelte';
import { mapMarkerStore } from '$lib/stores/mapMarker.svelte';
import { Debouncer } from '$lib/utils/debounce';
import { LngLat, type Map } from 'maplibre-gl';
import { getMapContext } from 'svelte-maplibre';
import { MapBouncer } from './MapBouncer';

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

class PanoramicLayerState {
	data = $state.raw<Collection | undefined>(undefined);
	fetchDebouncer = new Debouncer(200);
	moveBouncer = new MapBouncer();

	private reload = (map: Map) => {
		const bounds = map.getBounds();
		const zoom = map.getZoom();

		const { n, s, e, w } = extendBounds({
			n: bounds.getNorth(),
			s: bounds.getSouth(),
			e: bounds.getEast(),
			w: bounds.getWest()
		});

		this.fetchDebouncer.run(() => {
			getMapillaryGeoJSON(n, e, s, w, zoom >= 18, true)
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

	private handleMove = (map: Map) => {
		const bounds = map.getBounds();

		if (!this.moveBouncer.changed(bounds)) {
			console.debug('PanoramicLayer reload triggered, but map not moved.');
			return;
		}

		this.reload(map);
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
		const map = getMapContext()?.map;

		if (!map) {
			console.warn('Map not available, cannot display panoramas.');
			return;
		}

		const handleMove = () => this.handleMove(map);
		const reload = () => this.reload(map);

		mapBus.on('reload', reload);

		map.on('moveend', handleMove);
		map.on('zoomend', handleMove);
		reload();

		return () => {
			mapBus.off('reload', reload);
			map.off('moveend', handleMove);
			map.off('zoomend', handleMove);
		};
	};
}

export const panoramicLayerState = new PanoramicLayerState();
