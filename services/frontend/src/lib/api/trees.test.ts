import { describe, expect, it } from 'vitest';
import { getSearchTreesGeoJSON } from './trees';
import { config } from '$lib/env';

describe('getSearchTreesGeoJSON', () => {
	it('builds a search.json URL with the given bounds and query', () => {
		const url = getSearchTreesGeoJSON('liriodendron', 16, { n: 41, e: 45, s: 40, w: 44 });

		expect(url.startsWith(`${config.apiRoot}v1/trees/search.json?`)).toBe(true);

		const params = new URLSearchParams(url.split('?')[1]);

		expect(params.get('n')).toBe('41');
		expect(params.get('e')).toBe('45');
		expect(params.get('s')).toBe('40');
		expect(params.get('w')).toBe('44');
		expect(params.get('zoom')).toBe('16');
		expect(params.get('search')).toBe('liriodendron');
	});

	it('falls back to world bounds and the default zoom', () => {
		const url = getSearchTreesGeoJSON('');
		const params = new URLSearchParams(url.split('?')[1]);

		expect(url).toContain('v1/trees/search.json');
		expect(params.get('n')).toBe('90');
		expect(params.get('e')).toBe('180');
		expect(params.get('s')).toBe('-90');
		expect(params.get('w')).toBe('-180');
		expect(params.get('zoom')).toBe('10');
		expect(params.get('search')).toBe('');
	});
});
