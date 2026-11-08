import { get } from 'svelte/store';
import { beforeEach, describe, expect, it } from 'vitest';
import { mapLayerStore } from '$lib/stores/mapLayerStore';
import { LayerSelector } from './LayerSelector.svelte.ts';

const resetStore = (genus = true, center = false, stickyPoints = false) => {
	mapLayerStore.set({
		base: 'light',
		drone: false,
		alerts: true,
		panoramas: false,
		treeHints: false,
		water: true,
		genus,
		stickyPoints,
		center,
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

	it('reports sticky points as off when the crosshair is off even if stored as enabled', () => {
		resetStore(true, false, true);

		const selector = new LayerSelector();
		selector.init();

		expect(selector.center).toBe(false);
		expect(selector.stickyPoints).toBe(false);
	});

	it('reports sticky points as on when both are stored as enabled', () => {
		resetStore(true, true, true);

		const selector = new LayerSelector();
		selector.init();

		expect(selector.center).toBe(true);
		expect(selector.stickyPoints).toBe(true);
	});

	it('clears sticky points when the crosshair is turned off', () => {
		resetStore(true, true, true);

		const selector = new LayerSelector();
		selector.init();

		selector.toggleCenter();

		expect(selector.center).toBe(false);
		expect(selector.stickyPoints).toBe(false);
		expect(get(mapLayerStore).center).toBe(false);
		expect(get(mapLayerStore).stickyPoints).toBe(false);
	});

	it('does not toggle or persist sticky points while the crosshair is off', () => {
		resetStore(true, false, false);

		const selector = new LayerSelector();
		selector.init();

		selector.toggleStickyPoints();

		expect(selector.stickyPoints).toBe(false);
		expect(get(mapLayerStore).stickyPoints).toBe(false);
	});

	it('does not turn sticky points back on when the crosshair is re-enabled', () => {
		resetStore(true, false, false);

		const selector = new LayerSelector();
		selector.init();

		selector.toggleCenter();

		expect(selector.center).toBe(true);
		expect(selector.stickyPoints).toBe(false);
		expect(get(mapLayerStore).center).toBe(true);
		expect(get(mapLayerStore).stickyPoints).toBe(false);
	});
});
