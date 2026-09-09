import { goto, routes } from '$lib/routes';

class AddWaterState {
	public toggle = (e: Event) => {
		e.preventDefault();
		goto(routes.waterAdd());
	};

	public onMount = () => {
		return () => {};
	};
}

export const addWaterState = new AddWaterState();
