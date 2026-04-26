// This must go first for the mocks to work.
import { TREE_RESPONSE, mockedGoto } from './mocks';

import * as treesApi from '$lib/api/trees';
import type { IResponse, ITree } from '$lib/types';
import { get } from 'svelte/store';
import { describe, expect, it, vi } from 'vitest';
import { editor } from './hooks';

vi.mock('$lib/api/trees', () => ({
	getTree: vi.fn(),
	updateTreeLocation: vi.fn()
}));

describe('move-form/hooks', async () => {
	it('should call the update API', async () => {
		const tree_id = 'tree1';

		let update_called = false;

		vi.mocked(treesApi.getTree).mockResolvedValue({
			status: 200,
			data: TREE_RESPONSE
		});

		const { loading, value, reload, save } = editor(tree_id);

		expect(get(loading)).toBe(true);

		expect(get(value)).toStrictEqual({
			lat: 0,
			lng: 0
		});

		await reload(tree_id);
		expect(get(loading)).toBe(false);

		expect(get(value)).toEqual({
			lat: 1.23,
			lng: 2.34
		});

		vi.mocked(treesApi.updateTreeLocation).mockImplementation(
			async (id: string, lat: number, lon: number): Promise<IResponse<ITree>> => {
				expect(id).toEqual(tree_id);
				expect(lat).toEqual(1.23);
				expect(lon).toEqual(2.34);

				update_called = true;

				return {
					status: 200,
					data: TREE_RESPONSE
				};
			}
		);

		await save();
		expect(update_called).toBe(true);

		expect(mockedGoto).toHaveBeenCalledWith('/tree/tree1/preview');
	});
});
