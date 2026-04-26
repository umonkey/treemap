import { mapBus } from '$lib/buses/mapBus';
import { locationTracker } from './LocationTracker.svelte.ts';
import { locationStore } from '$lib/stores/locationStore';
import { get } from 'svelte/store';

class ComponentState {
	public handleLocate = () => {
		const currentPos = get(locationStore);
		if (currentPos) {
			mapBus.emit('move', {
				lat: currentPos.lat,
				lng: currentPos.lng
			});
		} else {
			locationTracker.start();
			const unsub = locationStore.subscribe((pos) => {
				if (pos) {
					mapBus.emit('move', {
						lat: pos.lat,
						lng: pos.lng
					});
					unsub();
				}
			});
		}
	};
}

export const componentState = new ComponentState();
