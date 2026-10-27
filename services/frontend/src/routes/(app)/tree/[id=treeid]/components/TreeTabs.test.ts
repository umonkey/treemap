import { describe, it, expect } from 'vitest';
import { activeTreeTab, treeTabs } from './TreeTabs.svelte.ts';

describe('treeTabs', () => {
	it('returns the tab definitions in display order', () => {
		expect(treeTabs('1').map((tab) => tab.id)).toEqual([
			'preview',
			'photos',
			'comments',
			'observations',
			'history'
		]);
	});
});

describe('activeTreeTab', () => {
	it('finds the photos tab and returns its suffix', () => {
		expect(activeTreeTab('/tree/1/photos', '1')?.suffix).toBe('/photos');
	});

	it('finds the preview tab and returns an empty suffix', () => {
		expect(activeTreeTab('/tree/1', '1')?.suffix).toBe('');
	});

	it('returns undefined for a non-tab sub-page', () => {
		expect(activeTreeTab('/tree/1/height', '1')).toBeUndefined();
	});

	it('returns undefined when the pathname belongs to another tree', () => {
		expect(activeTreeTab('/tree/2/photos', '1')).toBeUndefined();
	});
});
