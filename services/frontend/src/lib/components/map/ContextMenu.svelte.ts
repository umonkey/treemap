import { goto } from '$lib/routes';
import { mapMode } from '$lib/stores/mapMode';
import type { Map, MapMouseEvent, MapTouchEvent, Point } from 'maplibre-gl';
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

export class ContextMenuLogic {
	open = $state(false);

	private timer: ReturnType<typeof setTimeout> | null = null;
	private startPoint: { x: number; y: number } | null = null;
	private map: Map | null = null;

	private clearTimer = () => {
		if (this.timer !== null) {
			clearTimeout(this.timer);
			this.timer = null;
		}
	};

	public isPoi = (map: Map, point: Point): boolean => {
		const layers = POI_LAYERS.filter((id) => map.getLayer(id));

		if (layers.length === 0) {
			return false;
		}

		return map.queryRenderedFeatures(point, { layers }).length > 0;
	};

	public show = (map: Map, point: Point) => {
		const mode = get(mapMode);

		if (mode === 'add' || mode === 'add-row' || mode === 'move') {
			return;
		}

		if (this.isPoi(map, point)) {
			return;
		}

		this.open = true;
	};

	public handleContextMenu = (e: MapMouseEvent) => {
		if (e.originalEvent?.button !== 2) {
			return;
		}

		this.show(e.target, e.point);
	};

	public handleTouchStart = (e: MapTouchEvent) => {
		this.clearTimer();
		this.startPoint = { x: e.point.x, y: e.point.y };

		this.timer = setTimeout(() => {
			this.timer = null;
			document.getSelection()?.empty();
			this.show(e.target, e.point);
		}, 500);
	};

	public handleTouchMove = (e: MapTouchEvent) => {
		if (!this.startPoint) {
			return;
		}

		const dx = e.point.x - this.startPoint.x;
		const dy = e.point.y - this.startPoint.y;

		if (Math.sqrt(dx * dx + dy * dy) > 10) {
			this.clearTimer();
		}
	};

	public handleTouchEnd = () => {
		this.clearTimer();
	};

	public handleTouchCancel = () => {
		this.clearTimer();
	};

	public handleKeyDown = (e: KeyboardEvent) => {
		if (e.key === 'Escape') {
			this.handleClose();
		}
	};

	public handleClose = () => {
		this.open = false;
	};

	public handleNavigate = (url: string) => {
		this.open = false;
		goto(url);
	};

	public mount = (map: Map) => {
		this.map = map;
		map.on('contextmenu', this.handleContextMenu);
		map.on('touchstart', this.handleTouchStart);
		map.on('touchmove', this.handleTouchMove);
		map.on('touchend', this.handleTouchEnd);
		map.on('touchcancel', this.handleTouchCancel);
		map.on('movestart', this.handleClose);
	};

	public unmount = (map: Map) => {
		map.off('contextmenu', this.handleContextMenu);
		map.off('touchstart', this.handleTouchStart);
		map.off('touchmove', this.handleTouchMove);
		map.off('touchend', this.handleTouchEnd);
		map.off('touchcancel', this.handleTouchCancel);
		map.off('movestart', this.handleClose);
		this.clearTimer();
		this.map = null;
	};
}
