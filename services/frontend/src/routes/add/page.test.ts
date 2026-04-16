import { addTree } from '$lib/api/trees';
import { DEFAULT_TREE } from '$lib/constants';
import { routes } from '$lib/routes';
import { authStore } from '$lib/stores/authStore';
import type { IAddTreesRequest, IResponse, ITreeList } from '$lib/types';
import { cleanup, render, screen, waitFor } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest';
import Page from './+page.svelte';

vi.mock('$app/navigation', () => ({
	goto: vi.fn()
}));

vi.mock('$app/stores', () => ({
	page: {
		subscribe: (fn: (v: unknown) => void) => {
			fn({
				url: new URL('http://localhost/?lat=1.0&lng=2.0'),
				params: {}
			});
			return () => {};
		}
	}
}));

import { goto } from '$app/navigation';
const mockedGoto = vi.mocked(goto);

vi.mock('$lib/api/trees', () => ({
	addTree: vi.fn()
}));

beforeEach(() => {
	mockedGoto.mockClear();
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
						trees: [
							{
								...DEFAULT_TREE,
								id: 'added1'
							}
						],
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
	cleanup();
});

describe('add page', async () => {
	test('unauthorized', async () => {
		authStore.set(undefined);
		render(Page);

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

		render(Page);

		const em = screen.queryByRole('button', {
			name: /sign in with google/i
		});

		expect(em).toBeNull();
	});

	test('handle cancel', async () => {
		const user = userEvent.setup();
		authStore.set({
			token: 'secret',
			name: 'John Doe',
			picture: 'https://example.com/picture.jpg'
		});

		render(Page);

		const em = screen.getAllByRole('button', {
			name: /cancel/i
		})[0];

		await user.click(em);

		await waitFor(() => expect(mockedGoto).toHaveBeenCalledWith(routes.map()));
	});

	test('handle default submit', async () => {
		const user = userEvent.setup();
		authStore.set({
			token: 'secret',
			name: 'John Doe',
			picture: 'https://example.com/picture.jpg'
		});

		let request: IAddTreesRequest | null = null;

		vi.mocked(addTree).mockImplementation(
			async (req: IAddTreesRequest): Promise<IResponse<ITreeList>> => {
				request = req;

				return {
					status: 200,
					data: {
						trees: [DEFAULT_TREE],
						users: []
					},
					error: undefined
				};
			}
		);

		render(Page);

		const em = screen.getByRole('button', {
			name: /save changes/i
		});

		await user.click(em);

		expect((request as unknown as IAddTreesRequest)?.points[0]).toStrictEqual({
			lat: 1.0,
			lon: 2.0
		});

		await waitFor(() => expect(mockedGoto).toHaveBeenCalledWith(routes.mapPreview('tree1')));
	});

	test('handle submit with input', async () => {
		const user = userEvent.setup();
		authStore.set({
			token: 'secret',
			name: 'John Doe',
			picture: 'https://example.com/picture.jpg'
		});

		let request: IAddTreesRequest | null = null;

		const inputNumber = async (name: RegExp, value: string) => {
			const ctl = screen.getByRole('spinbutton', {
				name
			});

			await user.type(ctl, value);
		};

		render(Page);

		vi.mocked(addTree).mockImplementation(
			async (req: IAddTreesRequest): Promise<IResponse<ITreeList>> => {
				request = req;

				return {
					status: 200,
					data: {
						trees: [DEFAULT_TREE],
						users: []
					},
					error: undefined
				};
			}
		);

		await inputNumber(/year/i, '1980');

		const submit = screen.getByRole('button', {
			name: /save changes/i
		});
		await user.click(submit);

		expect(request).not.toBeNull();
		expect((request as unknown as IAddTreesRequest)?.year).toBe(1980);

		await waitFor(() => expect(mockedGoto).toHaveBeenCalledWith(routes.mapPreview('tree1')));
	});
});
