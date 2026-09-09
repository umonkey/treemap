import { goto, routes } from '$lib/routes';

export class AddWaterState {
	public toggle = (e: Event) => {
		e.preventDefault();
		goto(routes.waterAdd());
	};

	public onMount = () => {
		return () => {};
	};
}
