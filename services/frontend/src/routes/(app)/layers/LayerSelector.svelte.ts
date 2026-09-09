import { get } from 'svelte/store';
import { mapLayerStore } from '$lib/stores/mapLayerStore';

class SelectorState {
	base = $state<string>('light');
	drone = $state<boolean>(false);
	alerts = $state<boolean>(true);
	panoramas = $state<boolean>(false);
	treeHints = $state<boolean>(false);
	water = $state<boolean>(true);
	stickyPoints = $state<boolean>(true);

	public constructor() {
		const layers = get(mapLayerStore);
		this.base = layers.base ?? 'light';
		this.drone = layers.drone ?? false;
		this.alerts = layers.alerts ?? true;
		this.panoramas = layers.panoramas ?? false;
		this.treeHints = layers.treeHints ?? false;
		this.water = layers.water ?? true;
		this.stickyPoints = layers.stickyPoints ?? true;
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

	public toggleAlerts = () => {
		this.alerts = !this.alerts;

		mapLayerStore.update((store) => {
			store.alerts = this.alerts;
			return store;
		});
	};

	public togglePanoramas = () => {
		this.panoramas = !this.panoramas;

		mapLayerStore.update((store) => {
			store.panoramas = this.panoramas;
			return store;
		});
	};

	public toggleTreeHints = () => {
		this.treeHints = !this.treeHints;

		mapLayerStore.update((store) => {
			store.treeHints = this.treeHints;
			return store;
		});
	};

	public toggleWater = () => {
		this.water = !this.water;

		mapLayerStore.update((store) => {
			store.water = this.water;
			return store;
		});
	};

	public toggleStickyPoints = () => {
		this.stickyPoints = !this.stickyPoints;

		mapLayerStore.update((store) => {
			store.stickyPoints = this.stickyPoints;
			return store;
		});
	};
}

export const selectorState = new SelectorState();
