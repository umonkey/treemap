import { menuBus, type ITreeMenuEvent } from '$lib/buses/menuBus';

class MenuState {
	id = $state<string | undefined>(undefined);
	position = $state<{ x: number; y: number } | undefined>(undefined);

	private handleShow = (event: ITreeMenuEvent) => {
		this.id = event.id;

		this.position =
			event.x !== undefined && event.y !== undefined ? { x: event.x, y: event.y } : undefined;
	};

	public handleClose = () => {
		this.id = undefined;
		this.position = undefined;
	};

	public onMount = () => {
		menuBus.on('show', this.handleShow);

		return () => {
			menuBus.off('show', this.handleShow);
		};
	};
}

export const menuState = new MenuState();
