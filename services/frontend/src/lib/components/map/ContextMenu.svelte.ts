import { mapBus } from '$lib/buses/mapBus';
import { menuBus } from '$lib/buses/menuBus';
import { goto } from '$lib/routes';
import type { ILatLng } from '$lib/types';

export class ContextMenuLogic {
	open = $state(false);

	private coords: ILatLng | undefined = undefined;

	private handleShow = (coords: ILatLng) => {
		console.debug(`Showing menu for ${coords.lat},${coords.lng}`);
		this.coords = coords;
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
