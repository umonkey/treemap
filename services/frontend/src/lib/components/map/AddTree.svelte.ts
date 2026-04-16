import { mapBus } from '$lib/buses/mapBus';
import { type ILatLng } from '$lib/types';
import { goto, routes } from '$lib/routes';

class AddState {
	active = $state<boolean>(false);
	center = $state<ILatLng>();

	public toggle = (e: Event) => {
		e.preventDefault();
		this.active = !this.active;

		if (this.active) {
			mapBus.emit('preview', undefined);
		}
	};

	public handleConfirm = async () => {
		if (this.center) {
			this.active = false;
			await goto(routes.treeAdd(this.center.lat, this.center.lng));
		}
	};

	public handleCancel = () => {
		this.active = false;
	};

	private handleCenter = (ll: ILatLng) => {
		this.center = ll;
	};

	public onMount = () => {
		mapBus.on('center', this.handleCenter);
		mapBus.on('select', this.handleCancel);

		return () => {
			mapBus.off('center', this.handleCenter);
			mapBus.off('select', this.handleCancel);
		};
	};
}

export const addState = new AddState();
