import { mapBus } from '$lib/buses/mapBus';

class ButtonState {
	active = $state<boolean>(false);

	public toggle = () => {
		this.active = !this.active;
		mapBus.emit('preview', undefined);
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
