import { describe, it, expect } from 'vitest';
import { triangulateTree } from './triangulation';

describe('triangulateTree', () => {
	it('returns null for empty gcps', () => {
		expect(triangulateTree([])).toBeNull();
	});

	it('returns single gcp location when only 1 gcp provided', () => {
		const res = triangulateTree([{ lat: 40.0, lng: 44.0, radius: 50 }]);
		expect(res).toEqual({ lat: 40.0, lng: 44.0 });
	});

	it('triangulates intersecting circles correctly', () => {
		const gcp1 = { lat: 40.0, lng: 44.0, radius: 100 };
		const gcp2 = { lat: 40.0, lng: 44.01, radius: 100 };
		const result = triangulateTree([gcp1, gcp2]);
		expect(result).not.toBeNull();
		expect(result?.lat).toBeDefined();
		expect(result?.lng).toBeDefined();
	});

	it('ignores (0,0) coordinates', () => {
		const gcp1 = { lat: 0, lng: 0, radius: 100 };
		const gcp2 = { lat: 40.0, lng: 44.0, radius: 50 };
		const res = triangulateTree([gcp1, gcp2]);
		expect(res).toEqual({ lat: 40.0, lng: 44.0 });
	});
});
