import { getTree } from '$lib/api/trees';
import { DEFAULT_TREE } from '$lib/constants';
import type { IResponse, ISingleTree } from '$lib/types';
import { get } from 'svelte/store';
import { describe, expect, it, vi } from 'vitest';
import { loadTree } from './loadTree';

vi.mock('$lib/api/trees', () => ({
	getTree: vi.fn()
}));

describe('hooks/loadTree', async () => {
	it('should load a tree', async () => {
		vi.mocked(getTree).mockImplementation(async (): Promise<IResponse<ISingleTree>> => {
			return {
				status: 200,
				data: {
					...DEFAULT_TREE,
					users: []
				}
			};
		});

		const { loading, error, data, reload } = loadTree();
		expect(get(loading)).toBe(true);

		await reload('tree1');

		expect(get(loading)).toBe(false);
		expect(get(error)).toBeUndefined();
		expect(get(data)?.id).toEqual(DEFAULT_TREE.id);
	});

	it('should return an error', async () => {
		vi.mocked(getTree).mockImplementation(async (): Promise<IResponse<ISingleTree>> => {
			return {
				status: 500,
				data: undefined,
				error: {
					code: 'SomethingWentWrong',
					description: 'something went wrong'
				}
			};
		});

		const { loading, error, data, reload } = loadTree();
		expect(get(loading)).toBe(true);

		await reload('tree1');

		expect(get(loading)).toBe(false);
		expect(get(data)).toBeUndefined();
		expect(get(error)?.description).toEqual('something went wrong');
	});
});
