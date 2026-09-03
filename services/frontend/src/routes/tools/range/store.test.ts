import { describe, it, expect, beforeEach } from 'vitest';
import { RangeToolStore } from './store.svelte';

describe('RangeToolStore', () => {
	beforeEach(() => {
		localStorage.clear();
	});

	it('initializes with default values', () => {
		const store = new RangeToolStore();
		expect(store.gcps).toEqual([null, null, null, null]);
		expect(store.trees).toEqual([]);
		expect(store.validCount).toBe(0);
		expect(store.canContinue).toBe(false);
	});

	it('manages GCPs and derived states', () => {
		const store = new RangeToolStore();
		expect(store.canContinue).toBe(false);

		store.setGcp(0, { lat: 40.0, lng: 44.0 });
		expect(store.validCount).toBe(1);
		expect(store.canContinue).toBe(false);

		store.setGcp(1, { lat: 40.1, lng: 44.1 });
		expect(store.validCount).toBe(2);
		expect(store.canContinue).toBe(true);

		store.clearGcp(1);
		expect(store.validCount).toBe(1);
		expect(store.canContinue).toBe(false);
	});

	it('manages trees (add, remove, clear)', () => {
		const store = new RangeToolStore();
		expect(store.trees.length).toBe(0);

		store.addTree({ lat: 40.1872, lng: 44.5152 });
		expect(store.trees.length).toBe(1);
		expect(store.trees[0].lat).toBe(40.1872);
		expect(store.trees[0].lng).toBe(44.5152);

		const treeId = store.trees[0].id;
		store.addTree({ lat: 40.19, lng: 44.52 });
		expect(store.trees.length).toBe(2);

		store.removeTree(treeId);
		expect(store.trees.length).toBe(1);
		expect(store.trees[0].lat).toBe(40.19);

		store.clearTrees();
		expect(store.trees.length).toBe(0);
	});
});
