import { mapBus } from '$lib/buses/mapBus';
import { menuBus, type IMapMenuEvent } from '$lib/buses/menuBus';
import { goto } from '$lib/routes';
import type { ILatLng } from '$lib/types';

const MENU_MARGIN = 8;

export class ContextMenuLogic {
	open = $state(false);

	private coords: ILatLng | undefined = undefined;
	private position = $state<{ x: number; y: number } | undefined>(undefined);

	viewport = $state({ width: 0, height: 0 });
	menuSize = $state({ width: 0, height: 0 });

	clampedPosition = $derived.by(() => {
		if (!this.position) {
			return undefined;
		}

		const maxX = Math.max(MENU_MARGIN, this.viewport.width - this.menuSize.width - MENU_MARGIN);
		const maxY = Math.max(MENU_MARGIN, this.viewport.height - this.menuSize.height - MENU_MARGIN);

		return {
			x: Math.min(Math.max(this.position.x, MENU_MARGIN), maxX),
			y: Math.min(Math.max(this.position.y, MENU_MARGIN), maxY)
		};
	});

	private handleShow = (event: IMapMenuEvent) => {
		console.debug(`Showing menu for ${event.lat},${event.lng}`);
		this.coords = { lat: event.lat, lng: event.lng };
		this.position =
			event.x !== undefined && event.y !== undefined
				? { x: event.x, y: event.y }
				: { x: this.viewport.width / 2, y: this.viewport.height / 2 };
		this.open = true;
	};

	public handleKeyDown = (e: KeyboardEvent) => {
		if (e.key === 'Escape') {
			this.handleClose();
		}
	};

	public handleClose = () => {
		this.open = false;
	};

	public handleContextMenu = (e: MouseEvent) => {
		e.preventDefault();
		this.handleClose();
	};

	public handleNavigate = (url: string) => {
		this.open = false;

		if (this.coords) {
			mapBus.emit('move', this.coords);
		}

		goto(url);
	};

	public handleCopyCoordinates = async () => {
		if (!this.coords) {
			return;
		}

		try {
			await navigator.clipboard.writeText(`${this.coords.lat},${this.coords.lng}`);
		} catch (e) {
			console.error('Failed to copy coordinates to clipboard', e);
		} finally {
			this.open = false;
		}
	};

	public subscribe = () => {
		menuBus.on('showMap', this.handleShow);

		return () => {
			menuBus.off('showMap', this.handleShow);
		};
	};
}
