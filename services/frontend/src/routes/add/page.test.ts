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
				url: new URL('http://localhost/add'),
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

		// Even if unauthorized, the page should show (it might redirect later,
		// but the component itself should render its panel).
		screen.getByText(/add tree/i);
	});

	test('authorized', async () => {
		authStore.set({
			token: 'secret',
			id: 'user1',
			name: 'John Doe',
			picture: 'https://example.com/picture.jpg'
		});

		render(Page);
		screen.getByText(/add tree/i);
	});

	test('handle cancel', async () => {
		const user = userEvent.setup();
		authStore.set({
			token: 'secret',
			id: 'user1',
			name: 'John Doe',
			picture: 'https://example.com/picture.jpg'
		});

		render(Page);

		const em = screen.getByRole('button', {
			name: /cancel/i
		});

		await user.click(em);

		await waitFor(() => expect(mockedGoto).toHaveBeenCalledWith(routes.map()));
	});

	test('handle confirm submit', async () => {
		const user = userEvent.setup();
		authStore.set({
			token: 'secret',
			id: 'user1',
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
						trees: [{ ...DEFAULT_TREE, id: 'added1' }],
						users: []
					},
					error: undefined
				};
			}
		);

		render(Page);

		const em = screen.getByRole('button', {
			name: /confirm/i
		});

		await user.click(em);

		expect(request).not.toBeNull();
		// mapState default is 40.181389, 44.514444
		expect((request as unknown as IAddTreesRequest)?.points[0]).toStrictEqual({
			lat: 40.181389,
			lon: 44.514444
		});

		await waitFor(() => expect(mockedGoto).toHaveBeenCalledWith(routes.treeEdit('added1')));
	});
});
