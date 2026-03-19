import { get } from 'svelte/store';
import { mapLayerStore } from '$lib/stores/mapLayerStore';

class SelectorState {
	base = $state<string>('basic');
	drone = $state<boolean>(false);

	public constructor() {
		const layers = get(mapLayerStore);
		this.base = layers.base ?? 'basic';
		this.drone = layers.drone ?? false;
	}

	public setBase = (value: string) => {
		this.base = value;

		mapLayerStore.update((store) => {
			store.base = value;
			return store;
		});
	};

	public toggleDrone = () => {
		this.drone = !this.drone;

		mapLayerStore.update((store) => {
			store.drone = this.drone;
			return store;
		});
	};
}

export const selectorState = new SelectorState();
