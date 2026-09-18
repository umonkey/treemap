import { get } from 'svelte/store';
import { beforeEach, describe, expect, it } from 'vitest';
import { combineQuery, mapLayerStore, missingQuery } from './mapLayerStore';

const resetStore = () => {
	mapLayerStore.set({
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
	});
};

describe('combineQuery', () => {
	it('returns undefined when both parts are empty', () => {
		expect(combineQuery(undefined, '')).toBeUndefined();
		expect(combineQuery(null, '')).toBeUndefined();
		expect(combineQuery('', '')).toBeUndefined();
		expect(combineQuery('   ', '')).toBeUndefined();
	});

	it('returns the raw search when there are no missing filters', () => {
		expect(combineQuery('oak', '')).toBe('oak');
	});

	it('returns the missing filters when the raw search is empty', () => {
		expect(combineQuery('', 'no:height')).toBe('no:height');
		expect(combineQuery(undefined, 'no:height')).toBe('no:height');
	});

	it('combines the raw search and the missing filters', () => {
		expect(combineQuery('oak', 'no:height no:photo')).toBe('oak no:height no:photo');
	});
});

describe('missingQuery', () => {
	beforeEach(resetStore);

	it('returns an empty string when no flag is set', () => {
		expect(get(missingQuery)).toBe('');
	});

	it('maps each flag to its backend keyword', () => {
		mapLayerStore.update((store) => {
			store.missingHeight = true;
			return store;
		});
		expect(get(missingQuery)).toBe('no:height');

		resetStore();
		mapLayerStore.update((store) => {
			store.missingDiameter = true;
			return store;
		});
		expect(get(missingQuery)).toBe('no:diameter');

		resetStore();
		mapLayerStore.update((store) => {
			store.missingCircumference = true;
			return store;
		});
		expect(get(missingQuery)).toBe('no:circumference');

		resetStore();
		mapLayerStore.update((store) => {
			store.missingObservations = true;
			return store;
		});
		expect(get(missingQuery)).toBe('no:observations');

		resetStore();
		mapLayerStore.update((store) => {
			store.missingPhotos = true;
			return store;
		});
		expect(get(missingQuery)).toBe('no:photo');
	});

	it('joins multiple flags in a stable order', () => {
		mapLayerStore.update((store) => {
			store.missingPhotos = true;
			store.missingHeight = true;
			store.missingObservations = true;
			return store;
		});

		expect(get(missingQuery)).toBe('no:height no:observations no:photo');
	});
});
