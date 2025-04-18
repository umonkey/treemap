import Page from './+page.svelte';
import type { ITreeList } from '$lib/types';
import { DEFAULT_TREE } from '$lib/constants';
import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest';
import { authStore } from '$lib/stores/authStore';
import { cleanup, render, screen } from '@testing-library/svelte';

beforeEach(() => {
	const mockFetch = vi.fn();

	mockFetch.mockImplementation(async (req) => {
		// Simulate history.
		if (req.url === 'https://api.treemaps.app/v1/species/suggest') {
			return {
				ok: true,
				status: 200,
				json: async () => ['Ulmus', 'Elm', 'unknown']
			};
		}

		// Simulate search suggestions.
		if (req.url.startsWith('https://api.treemaps.app/v1/species/search?')) {
			return {
				ok: true,
				status: 200,
				json: async () => [
					{
						name: 'Ulmus',
						local: 'Elm'
					}
				]
			};
		}

		if (req.method === 'POST' && req.url === 'https://api.treemaps.app/v1/trees') {
			return {
				ok: true,
				status: 200,
				json: async () =>
					({
						trees: [DEFAULT_TREE],
						users: []
					}) as ITreeList
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

describe('add page', async () => {
	afterEach(cleanup);

	test('unauthorized', async () => {
		render(Page, {
			data: {
				lat: 1.0,
				lng: 2.0
			}
		});

		screen.getByRole('button', {
			name: /sign in with google/i
		});
	});

	test('authorized', async () => {
		authStore.set({
			token: 'secret',
			name: 'John Doe',
			picture: 'https://example.com/picture.jpg'
		});

		render(Page, {
			data: {
				lat: 1.0,
				lng: 2.0
			}
		});

		const em = screen.queryByRole('button', {
			name: /sign in with google/i
		});

		expect(em).toBeNull();
	});
});
