import AddForm from './AddForm.svelte';
import type { IAddTreesRequest, ITree, ITreeList } from '$lib/types';
import userEvent from '@testing-library/user-event';
import { DEFAULT_TREE } from '$lib/constants';
import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest';
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

describe('AddForm', async () => {
	afterEach(cleanup);

	test('handle cancel', async () => {
		const user = userEvent.setup();

		let clicked = false;

		render(AddForm, {
			lat: 1.0,
			lng: 2.0,
			onCancel: () => {
				clicked = true;
			}
		});

		const em = screen.getByRole('button', {
			name: /cancel/i
		});

		await user.click(em);

		expect(clicked).toBe(true);
	});

	// We allow adding trees with default values, all good.
	test('handle default submit', async () => {
		const user = userEvent.setup();

		let submitted: ITree | null = null;

		render(AddForm, {
			lat: 1.0,
			lng: 2.0,
			onAdded: (tree: ITree) => {
				submitted = tree;
			},
			onCancel: () => {}
		});

		const em = screen.getByRole('button', {
			name: /add tree/i
		});

		await user.click(em);

		expect(submitted?.id).toBe('tree1');
	});

	// We allow adding trees with default values, all good.
	test('handle submit with input', async () => {
		const user = userEvent.setup();

		let request: IAddTreesRequest | null = null;

		const inputNumber = async (name: RegExp, value: string) => {
			const ctl = screen.getByRole('spinbutton', {
				name
			});

			await user.type(ctl, value);
		};

		render(AddForm, {
			lat: 1.0,
			lng: 2.0,
			onAdded: () => {},
			onCancel: () => {},
			onBeforeSubmit: (req: IAddTreesRequest) => {
				request = req;
			}
		});

		await inputNumber(/year/i, '1980');

		const submit = screen.getByRole('button', {
			name: /add tree/i
		});
		await user.click(submit);

		expect(request).not.toBeNull();
		expect(request.year).toBe(1980);
	});
});
