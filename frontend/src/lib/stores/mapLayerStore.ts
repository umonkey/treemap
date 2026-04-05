import { ls } from '$lib/utils/localStorage';
import { derived, writable } from 'svelte/store';

interface IMapLayers {
	base: string | undefined;
	drone: boolean;
}

const getDefaultState = (): IMapLayers => {
	return {
		base: 'basic',
		drone: false
	};
};

export const mapLayerStore = writable<IMapLayers>(ls.read('mapLayerStore') ?? getDefaultState());

mapLayerStore.subscribe((value: IMapLayers) => {
	ls.write('mapLayerStore', value);
});

export const baseLayer = derived(mapLayerStore, ($mapStore) => $mapStore?.base);
export const droneLayer = derived(mapLayerStore, ($mapStore) => $mapStore?.drone);
