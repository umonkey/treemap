import type { LngLat } from 'maplibre-gl';
import { goto, routes } from '$lib/routes';

class AddState {
	active = $state<boolean>(false);

	public toggle = (e: Event) => {
		e.preventDefault();
		this.active = !this.active;
	};

	public handleConfirm = async (ll: LngLat) => {
		this.active = false;
		await goto(routes.treeAdd(ll.lat, ll.lng));
	};

	public handleCancel = () => {
		this.active = false;
	};
}

export const addState = new AddState();
