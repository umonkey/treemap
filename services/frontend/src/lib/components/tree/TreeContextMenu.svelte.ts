import { menuBus } from '$lib/buses/menuBus';

class MenuState {
	id = $state<string | undefined>(undefined);

	private handleShow = (id: string) => {
		this.id = id;
	};

	public handleClose = () => {
		this.id = undefined;
	};

	public onMount = () => {
		menuBus.on('show', this.handleShow);

		return () => {
			menuBus.off('show', this.handleShow);
		};
	};
}

export const menuState = new MenuState();
