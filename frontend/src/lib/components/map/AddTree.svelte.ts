import type { LngLat } from 'maplibre-gl';

export type ClickFn = (ll: LngLat) => void;

class AddState {
	active = $state<boolean>(false);
	onConfirm = $state<ClickFn>(() => {});

	public toggle = (e: Event) => {
		e.preventDefault();
		this.active = !this.active;
	};

	public handleConfirm = (ll: LngLat) => {
		this.active = false;
		this.onConfirm(ll);
	};

	public handleCancel = () => {
		this.active = false;
	};
}

export const addState = new AddState();
