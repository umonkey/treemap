import { menuBus } from '$lib/buses/menuBus';
import { mapMode } from '$lib/stores/mapMode';
import type { LngLat, Map, MapMouseEvent, MapTouchEvent, Point } from 'maplibre-gl';
import { get } from 'svelte/store';

const POI_LAYERS = [
	'tree-crowns-small',
	'tree-crowns-large',
	'tree-trunks',
	'panoramas-images',
	'panoramas-sequences',
	'water-source-discs',
	'water-source-dots',
	'chatbot-alerts'
];

export class MapMenuBridgeLogic {
	private menuTimer: ReturnType<typeof setTimeout> | null = null;
	private menuStartPoint: { x: number; y: number } | null = null;

	private clearMenuTimer = () => {
		if (this.menuTimer !== null) {
			clearTimeout(this.menuTimer);
			this.menuTimer = null;
		}
	};

	public isPoi = (map: Map, point: Point): boolean => {
		const layers = POI_LAYERS.filter((id) => map.getLayer(id));

		if (layers.length === 0) {
			return false;
		}

		return map.queryRenderedFeatures(point, { layers }).length > 0;
	};

	public showMapMenu = (map: Map, point: Point, lngLat: LngLat) => {
		const mode = get(mapMode);

		if (this.isPoi(map, point)) {
			return;
		}

		menuBus.emit('showMap', { lat: lngLat.lat, lng: lngLat.lng });
	};

	private handleContextMenu = (e: MapMouseEvent) => {
		if (e.originalEvent?.button !== 2) {
			return;
		}

		this.showMapMenu(e.target, e.point, e.lngLat);
	};

	private handleTouchStart = (e: MapTouchEvent) => {
		this.clearMenuTimer();
		this.menuStartPoint = { x: e.point.x, y: e.point.y };

		this.menuTimer = setTimeout(() => {
			this.menuTimer = null;
			document.getSelection()?.empty();
			this.showMapMenu(e.target, e.point, e.lngLat);
		}, 500);
	};

	private handleTouchMove = (e: MapTouchEvent) => {
		if (!this.menuStartPoint) {
			return;
		}

		const dx = e.point.x - this.menuStartPoint.x;
		const dy = e.point.y - this.menuStartPoint.y;

		if (Math.sqrt(dx * dx + dy * dy) > 10) {
			this.clearMenuTimer();
		}
	};

	private handleTouchEnd = () => {
		this.clearMenuTimer();
	};

	private handleTouchCancel = () => {
		this.clearMenuTimer();
	};

	public mount = (map: Map) => {
		map.on('contextmenu', this.handleContextMenu);
		map.on('touchstart', this.handleTouchStart);
		map.on('touchmove', this.handleTouchMove);
		map.on('touchend', this.handleTouchEnd);
		map.on('touchcancel', this.handleTouchCancel);
	};

	public unmount = (map: Map) => {
		map.off('contextmenu', this.handleContextMenu);
		map.off('touchstart', this.handleTouchStart);
		map.off('touchmove', this.handleTouchMove);
		map.off('touchend', this.handleTouchEnd);
		map.off('touchcancel', this.handleTouchCancel);
		this.clearMenuTimer();
		this.menuStartPoint = null;
	};
}
