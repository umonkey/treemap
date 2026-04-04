import { mapBus } from '$lib/buses/mapBus';
import type { ILatLng } from '$lib/types';

type MoveFn = (ll: ILatLng) => void;

class PickerState {
	onMove: MoveFn = () => {};

	public handleMove = (ll: ILatLng) => {
		this.onMove(ll);
	};

	public handleCenter = (ll: ILatLng) => {
		mapBus.emit('center', ll);
	};
}

export const pickerState = new PickerState();
