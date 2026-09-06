import { describe, it, expect } from 'vitest';
import { getOffsetDelta, calculateDestination, roundOffset } from './geo';

describe('geo utils', () => {
	it('computes offset delta correctly for 0.1m at equator', () => {
		const delta = getOffsetDelta(0, 0.1);
		expect(delta.deltaLat).toBeCloseTo(0.1 / 111320, 10);
		expect(delta.deltaLon).toBeCloseTo(0.1 / 111320, 10);
	});

	it('computes offset delta correctly at 40 degrees latitude', () => {
		const delta = getOffsetDelta(40, 0.1);
		expect(delta.deltaLat).toBeCloseTo(0.1 / 111320, 10);
		expect(delta.deltaLon).toBeGreaterThan(delta.deltaLat);
	});

	it('calculates destination point correctly', () => {
		const [lng, lat] = calculateDestination(40.0, 44.0, 0, 1000);
		expect(lat).toBeGreaterThan(40.0);
		expect(lng).toBeCloseTo(44.0, 4);
	});

	it('rounds offset correctly', () => {
		expect(roundOffset(-0.000003779734099892205, 7)).toBe(-0.0000038);
		expect(roundOffset(0.123456789, 7)).toBe(0.1234568);
		expect(roundOffset(Number.NaN, 7)).toBe(0);
		// @ts-expect-error testing invalid input
		expect(roundOffset(null, 7)).toBe(0);
	});
});
