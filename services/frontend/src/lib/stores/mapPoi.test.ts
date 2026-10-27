import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import { mapPoiStore } from './mapPoi.svelte';
import { getDistance } from '$lib/utils/geo';
import type { ILatLng } from '$lib/types';

const center: ILatLng = { lat: 40.0, lng: 44.0 };

const near = { lat: 40.001, lon: 44.0, url: 'near' };
const mid = { lat: 40.002, lon: 44.0, url: 'mid' };
const far = { lat: 40.01, lon: 44.0, url: 'far' };

describe('mapPoiStore.getNearestTrees', () => {
	beforeEach(() => {
		mapPoiStore.trees = [];
	});

	afterEach(() => {
		mapPoiStore.trees = [];
	});

	it('returns trees sorted ascending by distance from the center', () => {
		mapPoiStore.trees = [far, mid, near];

		const result = mapPoiStore.getNearestTrees(center, 3);

		expect(result.map((r) => r.poi.url)).toEqual(['near', 'mid', 'far']);
		expect(result[0].distance).toBeLessThan(result[1].distance);
		expect(result[1].distance).toBeLessThan(result[2].distance);
		expect(result[0].distance).toBe(getDistance(center, { lat: near.lat, lng: near.lon }));
	});

	it('slices to the N nearest trees', () => {
		mapPoiStore.trees = [far, mid, near];

		const result = mapPoiStore.getNearestTrees(center, 2);

		expect(result.map((r) => r.poi.url)).toEqual(['near', 'mid']);
	});

	it('filters out trees beyond maxDistance', () => {
		mapPoiStore.trees = [far, mid, near];
		const maxDistance = getDistance(center, { lat: mid.lat, lng: mid.lon });

		const result = mapPoiStore.getNearestTrees(center, 3, maxDistance);

		expect(result.map((r) => r.poi.url)).toEqual(['near', 'mid']);
	});

	it('returns only the single tree within range when one of two is too far', () => {
		mapPoiStore.trees = [mid, near];
		const maxDistance = getDistance(center, { lat: near.lat, lng: near.lon });

		const result = mapPoiStore.getNearestTrees(center, 3, maxDistance);

		expect(result).toHaveLength(1);
		expect(result[0].poi.url).toBe('near');
	});

	it('returns an empty array when there are no trees', () => {
		const result = mapPoiStore.getNearestTrees(center, 3);

		expect(result).toEqual([]);
	});

	it('returns an empty array when count is zero or negative', () => {
		mapPoiStore.trees = [near, mid, far];

		expect(mapPoiStore.getNearestTrees(center, 0)).toEqual([]);
		expect(mapPoiStore.getNearestTrees(center, -1)).toEqual([]);
	});
});

describe('mapPoiStore tree tab suffix', () => {
	afterEach(() => {
		mapPoiStore.trees = [];
		mapPoiStore.water = [];
		mapPoiStore.treeTab = '';
	});

	it('appends the open tree tab to tree POI URLs', () => {
		mapPoiStore.treeTab = '/photos';
		mapPoiStore.trees = [{ lat: 40.001, lon: 44.0, url: '/tree/1' }];

		expect(mapPoiStore.getNearest(center)?.poi.url).toBe('/tree/1/photos');
	});

	it('does not mutate the base tree URLs', () => {
		mapPoiStore.treeTab = '/photos';
		mapPoiStore.trees = [{ lat: 40.001, lon: 44.0, url: '/tree/1' }];

		mapPoiStore.getNearest(center);

		expect(mapPoiStore.trees[0].url).toBe('/tree/1');
	});

	it('leaves water POI URLs unchanged', () => {
		mapPoiStore.treeTab = '/photos';
		mapPoiStore.trees = [{ lat: 40.01, lon: 44.0, url: '/tree/1' }];
		mapPoiStore.water = [{ lat: 40.0005, lon: 44.0, url: '/water/9' }];

		expect(mapPoiStore.getNearest(center)?.poi.url).toBe('/water/9');
		expect(mapPoiStore.pois.map((poi) => poi.url)).toEqual(['/tree/1/photos', '/water/9']);
	});

	it('leaves tree POI URLs unchanged when no tree tab is open', () => {
		mapPoiStore.treeTab = '';
		mapPoiStore.trees = [{ lat: 40.001, lon: 44.0, url: '/tree/1' }];

		expect(mapPoiStore.getNearest(center)?.poi.url).toBe('/tree/1');
	});
});
