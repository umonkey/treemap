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
	water: boolean;
	stickyPoints: boolean;
	center: boolean;
	missingHeight: boolean;
	missingDiameter: boolean;
	missingCircumference: boolean;
	missingObservations: boolean;
	missingPhotos: boolean;
}

const getDefaultState = (): IMapLayers => {
	return {
		base: 'light',
		drone: false,
		alerts: true,
		panoramas: false,
		treeHints: false,
		water: true,
		stickyPoints: false,
		center: false,
		missingHeight: false,
		missingDiameter: false,
		missingCircumference: false,
		missingObservations: false,
		missingPhotos: false
	};
};

const getInitialState = (): IMapLayers => ({
	...getDefaultState(),
	...ls.read<Partial<IMapLayers>>('mapLayerStore')
});

export const mapLayerStore = writable<IMapLayers>(getInitialState());

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
export const waterLayer = derived(mapLayerStore, ($mapStore) => $mapStore?.water);
export const stickyPointsLayer = derived(mapLayerStore, ($mapStore) => $mapStore?.stickyPoints);
export const centerLayer = derived(mapLayerStore, ($mapStore) => $mapStore?.center === true);

export const missingQuery = derived(mapLayerStore, ($l) => {
	const parts: string[] = [];
	if ($l.missingHeight) parts.push('no:height');
	if ($l.missingDiameter) parts.push('no:diameter');
	if ($l.missingCircumference) parts.push('no:circumference');
	if ($l.missingObservations) parts.push('no:observations');
	if ($l.missingPhotos) parts.push('no:photo');
	return parts.join(' ');
});

export const combineQuery = (
	search: string | null | undefined,
	missing: string
): string | undefined => {
	const combined = [search, missing].filter(Boolean).join(' ').trim();
	return combined || undefined;
};
