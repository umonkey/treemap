import { mapBus } from '$lib/buses/mapBus';
import { goto } from '$lib/routes';
import { mapLayerStore } from '$lib/stores/mapLayerStore';
import { mapPoiStore } from '$lib/stores/mapPoi.svelte';
import type { ILatLng } from '$lib/types';
import type { Map, MapLibreEvent } from 'maplibre-gl';
import { get } from 'svelte/store';

export class NearestPoiLogic {
	private map: Map | undefined;
	private zoomChanged = false;

	center = $state<ILatLng>();
	maxDistance = $state<number>(100);
	zoom = $state<number>(0);
	moving = $state(false);

	visible = $derived(this.zoom >= 18);

	nearestPoi = $derived(
		this.center ? mapPoiStore.getNearest(this.center, this.maxDistance) : undefined
	);

	links = $derived.by(() => {
		const result = this.nearestPoi;
		const center = this.center;

		if (!result || !center) {
			return [];
		}

		const { poi, distance } = result;

		return [
			{
				poi,
				distance,
				midpoint: [(center.lng + poi.lon) / 2, (center.lat + poi.lat) / 2] as [number, number],
				line: {
					type: 'Feature' as const,
					geometry: {
						type: 'LineString' as const,
						coordinates: [
							[center.lng, center.lat],
							[poi.lon, poi.lat]
						]
					},
					properties: {}
				}
			}
		];
	});

	private handleMoveStart = () => {
		this.zoomChanged = false;
	};

	private handleZoom = () => {
		this.zoomChanged = true;
	};

	private handleMove = () => {
		if (!this.map) {
			return;
		}

		this.moving = true;

		const center = this.map.getCenter();
		this.center = { lat: center.lat, lng: center.lng };
	};

	private handleMoveEnd = (e: MapLibreEvent) => {
		this.moving = false;

		if (!this.map) {
			return;
		}

		this.zoom = this.map.getZoom();

		const center = this.map.getCenter();
		this.center = { lat: center.lat, lng: center.lng };

		if (!e.originalEvent || this.zoomChanged) {
			return;
		}

		if (!get(mapLayerStore).stickyPoints || !this.visible) {
			return;
		}

		const result = mapPoiStore.getNearest(this.center, this.maxDistance);
		if (!result) {
			return;
		}

		const { poi } = result;
		console.debug(`[nearest] Snapping to nearest POI at {poi.lat},{poi.lon}`);

		mapBus.emit('pin', { lat: poi.lat, lng: poi.lon });
		mapBus.emit('move', { lat: poi.lat, lng: poi.lon });

		goto(poi.url);
	};

	public mount = (map: Map) => {
		this.map = map;

		const center = map.getCenter();
		this.center = { lat: center.lat, lng: center.lng };

		map.on('movestart', this.handleMoveStart);
		map.on('zoom', this.handleZoom);
		map.on('move', this.handleMove);
		map.on('moveend', this.handleMoveEnd);
	};

	public unmount = () => {
		this.map?.off('movestart', this.handleMoveStart);
		this.map?.off('zoom', this.handleZoom);
		this.map?.off('move', this.handleMove);
		this.map?.off('moveend', this.handleMoveEnd);
		this.map = undefined;
	};
}
