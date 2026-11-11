import { getPanoramaFiles, type PanoramaFile } from '$lib/api/panoramas';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { PageState } from './page.svelte.ts';

vi.mock('$lib/api/panoramas', () => ({
	getPanoramaFiles: vi.fn()
}));

const mockedGetPanoramaFiles = vi.mocked(getPanoramaFiles);

const logFile: PanoramaFile = {
	name: 'processing.log',
	url: 'https://example.com/42/processing.log',
	size: 2048
};

const imageFile: PanoramaFile = {
	name: 'b.jpg',
	url: 'https://example.com/42/b.jpg',
	size: 1024
};

const anotherImageFile: PanoramaFile = {
	name: 'a.jpg',
	url: null,
	size: 512
};

describe('panorama files PageState', () => {
	beforeEach(() => {
		mockedGetPanoramaFiles.mockReset();
	});

	it('loads the files of a panorama', async () => {
		mockedGetPanoramaFiles.mockResolvedValueOnce({
			status: 200,
			data: [logFile, imageFile]
		});

		const state = new PageState();

		await state.reload('42');

		expect(mockedGetPanoramaFiles).toHaveBeenCalledWith('42');
		expect(state.files).toEqual([logFile, imageFile]);
		expect(state.error).toBeUndefined();
		expect(state.isLoading).toBe(false);
	});

	it('sorts non-image files first and then alphabetically', async () => {
		mockedGetPanoramaFiles.mockResolvedValueOnce({
			status: 200,
			data: [imageFile, anotherImageFile, logFile]
		});

		const state = new PageState();

		await state.reload('42');

		expect(state.sortedFiles).toEqual([logFile, anotherImageFile, imageFile]);
	});

	it('stores the API error and clears the files when loading fails', async () => {
		mockedGetPanoramaFiles.mockResolvedValueOnce({
			status: 404,
			error: { code: 'NotFound', description: 'Panorama not found.' }
		});

		const state = new PageState();

		await state.reload('42');

		expect(state.error).toEqual({ code: 'NotFound', description: 'Panorama not found.' });
		expect(state.files).toEqual([]);
		expect(state.isLoading).toBe(false);
	});
});
