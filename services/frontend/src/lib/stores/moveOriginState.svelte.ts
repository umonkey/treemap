import type { ILatLng } from '$lib/types';

class MoveOriginState {
	origin = $state<ILatLng | undefined>(undefined);
}

export const moveOriginState = new MoveOriginState();
