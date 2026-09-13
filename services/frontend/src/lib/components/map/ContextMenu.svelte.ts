import { menuBus } from '$lib/buses/menuBus';
import { goto } from '$lib/routes';
import type { ILatLng } from '$lib/types';

export class ContextMenuLogic {
	open = $state(false);

	private handleShow = (coords: ILatLng) => {
		console.debug(`Showing menu for ${coords.lat},${coords.lng}`);
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
		goto(url);
	};

	public onMount = () => {
		menuBus.on('showMap', this.handleShow);

		return () => {
			menuBus.off('showMap', this.handleShow);
		};
	};
}
