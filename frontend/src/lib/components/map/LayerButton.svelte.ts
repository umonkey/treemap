import { mapBus } from '$lib/buses/mapBus';
import { goto, routes } from '$lib/routes';

class ButtonState {
	public toggle = async () => {
		mapBus.emit('preview', undefined);
		await goto(routes.layers());
	};
}

export const buttonState = new ButtonState();
