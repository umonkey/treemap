import { mapBus } from '$lib/buses/mapBus';
import type { ILatLng } from '$lib/types';

export class MarkerLogic {
	pin = $state<ILatLng | undefined>(undefined);

	public get center(): ILatLng | undefined {
		return this.pin;
	}

	constructor() {
		mapBus.on('pin', this.handlePin);
	}

	handlePin = (ll: ILatLng | undefined) => {
		this.pin = ll;
	};

	destroy = () => {
		mapBus.off('pin', this.handlePin);
	};
}
