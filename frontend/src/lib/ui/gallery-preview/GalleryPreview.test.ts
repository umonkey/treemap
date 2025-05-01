import { cleanup, render, screen } from '@testing-library/svelte';
import { afterEach, beforeEach, describe, expect, vi, test } from 'vitest';
import GalleryPreview from './GalleryPreview.svelte';
import { DEFAULT_TREE } from '$lib/constants';
import { modeStore } from '$lib/stores/modeStore';
import { ModeEnum } from '$lib/enums';

beforeEach(() => {
	const mockFetch = vi.fn();

	mockFetch.mockImplementation(async (req) => {
		// A tree with no images.
		if (req.url === 'https://api.treemaps.app/v1/trees/tree-with-no-images') {
			return {
				ok: true,
				status: 200,
				json: async () => ({
					...DEFAULT_TREE,
					id: 'tree-with-no-images',
					files: []
				})
			};
		}

		console.warn(`[test] Failing an unexpected fetch call: ${req.url}`);

		throw new Error(`Unexpected fetch call: ${req.url}`);
	});

	vi.stubGlobal('fetch', mockFetch);
});

afterEach(() => {
	vi.unstubAllGlobals();
});

describe('GalleryPreview', async () => {
	afterEach(cleanup);

	test('have a default image in explorer mode', async () => {
		modeStore.set(ModeEnum.Explorer);

		const tree = {
			...DEFAULT_TREE,
			id: 'tree-with-no-images',
			files: []
		};

		render(GalleryPreview, {
			id: tree.id
		});

		const image = await screen.findByTitle('No photos of this tree.');
		expect(image.getAttribute('href')).toBe('/tree/tree-with-no-images');
	});

	test('have an upload link in mapper mode', async () => {
		modeStore.set(ModeEnum.Mapper);

		const tree = {
			...DEFAULT_TREE,
			id: 'tree-with-no-images',
			files: []
		};

		render(GalleryPreview, {
			id: tree.id,
		});

		const link = await screen.findByTitle('Upload a new image');
		expect(link.getAttribute('href')).toBe('/tree/tree-with-no-images/upload');
	});
});
