import { mapBus } from '$lib/buses/mapBus';
import { goto, routes } from '$lib/routes';

class ButtonState {
	active = $state<boolean>(false);

	public toggle = async () => {
		this.active = !this.active;
		mapBus.emit('preview', undefined);
		await goto(routes.layers());
	};

	public ignoreClick = (e: Event) => {
		e.stopPropagation();
		e.preventDefault();
	};

	public close = () => {
		this.active = false;
	};
}

export const buttonState = new ButtonState();
