import type { ITree, IResponse, ISingleTree, IChangeList } from '$lib/types';
import { DEFAULT_TREE } from '$lib/constants';
import { apiClient } from '$lib/api';
import { beforeEach, afterEach, describe, expect, it, vi } from 'vitest';
import { get } from 'svelte/store';
import { goto } from '$app/navigation';
import { editor } from './hooks';

const TREE_RESPONSE = {
	...DEFAULT_TREE,
	id: 'tree1',
	height: 1.23
} as ISingleTree;

const HISTORY_RESPONSE = {
	props: [
		{
			id: 'change1',
			tree_id: 'tree1',
			name: 'diameter',
			value: '1',
			added_at: 0,
			added_by: 'user1'
		}
	],
	users: []
} as IChangeList;

vi.mock('$app/navigation', async () => {
	return {
		goto: vi.fn()
	};
});

const mockedGoto = vi.mocked(goto);

beforeEach(() => {
	const mockFetch = vi.fn();

	mockFetch.mockImplementation(async (req) => {
		// Get tree info.
		if (req.url === 'https://api.treemaps.app/v1/trees/tree1') {
			return {
				ok: true,
				status: 200,
				json: async () => TREE_RESPONSE
			};
		}

		// Simulate change history.
		if (req.url === 'https://api.treemaps.app/v1/trees/tree1/history') {
			return {
				ok: true,
				status: 200,
				json: async () => HISTORY_RESPONSE
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

describe('height-form/hooks', async () => {
	it('should call the update API', async () => {
		const tree = {
			...DEFAULT_TREE,
			id: 'tree1',
			height: 1.23
		};

		let update_called = false;

		const { loading, value, history, reload, save } = editor(tree.id);

		expect(get(loading)).toBe(true);
		expect(get(value)).toBe(0.0);
		expect(get(history)).toHaveLength(0);

		await reload(tree.id);
		expect(get(loading)).toBe(false);
		expect(get(value)).toEqual(1.23);
		expect(get(history)).toHaveLength(1);

		apiClient.updateTreeHeight = async (id: string, value: number): Promise<IResponse<ITree>> => {
			expect(id).toEqual(tree.id);
			expect(value).toEqual(2.34);

			update_called = true;

			return {
				status: 200,
				data: tree
			};
		};

		value.set(2.34);

		await save();
		expect(update_called).toBe(true);

		expect(mockedGoto).toHaveBeenCalledWith('/map?preview=tree1');
	});
});
