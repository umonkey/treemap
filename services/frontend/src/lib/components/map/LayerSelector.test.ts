import { get } from 'svelte/store';
import { beforeEach, describe, expect, it } from 'vitest';
import { mapLayerStore } from '$lib/stores/mapLayerStore';
import { LayerSelector } from './LayerSelector.svelte.ts';

const resetStore = (genus: boolean) => {
	mapLayerStore.set({
		base: 'light',
		drone: false,
		alerts: true,
		panoramas: false,
		treeHints: false,
		water: true,
		genus,
		stickyPoints: false,
		center: false,
		missingHeight: false,
		missingDiameter: false,
		missingCircumference: false,
		missingObservations: false,
		missingPhotos: false
	});
};

describe('LayerSelector', () => {
	beforeEach(() => resetStore(true));

	it('initializes genus labels as enabled', () => {
		const selector = new LayerSelector();
		selector.init();

		expect(selector.genus).toBe(true);
	});

	it('falls back to enabled when the stored state predates the flag', () => {
		mapLayerStore.update((store) => {
			(store as Partial<typeof store>).genus = undefined;
			return store;
		});

		const selector = new LayerSelector();
		selector.init();

		expect(selector.genus).toBe(true);
	});

	it('toggles genus labels in the store', () => {
		const selector = new LayerSelector();
		selector.init();

		selector.toggleGenus();

		expect(selector.genus).toBe(false);
		expect(get(mapLayerStore).genus).toBe(false);

		selector.toggleGenus();

		expect(selector.genus).toBe(true);
		expect(get(mapLayerStore).genus).toBe(true);
	});
});
