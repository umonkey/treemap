import { DEFAULT_BBOX } from '$lib/constants';
import { getSettings } from '$lib/api/settings';
import { bboxCenter } from '$lib/utils/geo';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { settingsState } from './settings.svelte';

vi.mock('$lib/api/settings', () => ({
	getSettings: vi.fn()
}));

const mockedGetSettings = vi.mocked(getSettings);

describe('settingsState', () => {
	beforeEach(() => {
		settingsState.name = '';
		settingsState.description = null;
		settingsState.bbox = DEFAULT_BBOX;
		settingsState.loaded = false;

		mockedGetSettings.mockReset();
	});

	it('starts with the default bbox before loading', () => {
		expect(settingsState.bbox).toEqual(DEFAULT_BBOX);
		expect(settingsState.center).toEqual(bboxCenter(DEFAULT_BBOX));
		expect(settingsState.name).toBe('');
		expect(settingsState.loaded).toBe(false);
	});

	it('stores the instance settings once loaded', async () => {
		mockedGetSettings.mockResolvedValue({
			status: 200,
			data: { name: 'Trees of Yerevan', description: 'The map.', bbox: [1, 2, 3, 4] }
		});

		await settingsState.load();

		expect(settingsState.name).toBe('Trees of Yerevan');
		expect(settingsState.description).toBe('The map.');
		expect(settingsState.bbox).toEqual([1, 2, 3, 4]);
		expect(settingsState.center).toEqual(bboxCenter([1, 2, 3, 4]));
		expect(settingsState.loaded).toBe(true);
	});

	it('keeps the default bbox when the instance has none', async () => {
		mockedGetSettings.mockResolvedValue({
			status: 200,
			data: { name: 'No Bbox', description: null, bbox: null }
		});

		await settingsState.load();

		expect(settingsState.name).toBe('No Bbox');
		expect(settingsState.bbox).toEqual(DEFAULT_BBOX);
		expect(settingsState.loaded).toBe(true);
	});

	it('keeps the defaults when loading fails', async () => {
		mockedGetSettings.mockResolvedValue({
			status: 500,
			error: { code: 'instance_not_found', description: 'Nope.' }
		});

		await settingsState.load();

		expect(settingsState.bbox).toEqual(DEFAULT_BBOX);
		expect(settingsState.loaded).toBe(true);
	});
});
