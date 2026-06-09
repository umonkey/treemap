import { showError } from '$lib/errors';
import { locale } from '$lib/locale';
import { ls } from '$lib/utils/localStorage';
import { derived, writable } from 'svelte/store';

interface IMapLayers {
	base: string | undefined;
	drone: boolean;
	alerts: boolean;
	panoramas: boolean;
	treeHints: boolean;
	stickyPoints: boolean;
}

const getDefaultState = (): IMapLayers => {
	return {
		base: 'basic',
		drone: false,
		alerts: true,
		panoramas: false,
		treeHints: false,
		stickyPoints: true
	};
};

export const mapLayerStore = writable<IMapLayers>(ls.read('mapLayerStore') ?? getDefaultState());

mapLayerStore.subscribe((value: IMapLayers) => {
	if (!ls.write('mapLayerStore', value)) {
		showError(locale.toastStorageError());
	}
});

export const baseLayer = derived(mapLayerStore, ($mapStore) => $mapStore?.base);
export const droneLayer = derived(mapLayerStore, ($mapStore) => $mapStore?.drone);
export const alertsLayer = derived(mapLayerStore, ($mapStore) => $mapStore?.alerts);
export const panoramasLayer = derived(mapLayerStore, ($mapStore) => $mapStore?.panoramas);
export const treeHintsLayer = derived(mapLayerStore, ($mapStore) => $mapStore?.treeHints);
export const stickyPointsLayer = derived(mapLayerStore, ($mapStore) => $mapStore?.stickyPoints);
