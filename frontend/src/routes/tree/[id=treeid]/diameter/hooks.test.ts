// This must go first for the mocks to work.
import { mockedGoto, TREE_RESPONSE } from './mocks';

import type { ITree, IResponse } from '$lib/types';
import { apiClient } from '$lib/api';
import { describe, expect, it } from 'vitest';
import { editor } from './hooks';
import { get } from 'svelte/store';

describe('crown-form/hooks', async () => {
	it('should call the update API', async () => {
		let update_called = false;

		apiClient.getTree = async () => ({
			status: 200,
			data: TREE_RESPONSE
		});

		const { loading, value, reload, save } = editor('tree1');

		expect(get(loading)).toBe(true);
		expect(get(value)).toBe(0.0);

		await reload('tree1');
		expect(get(loading)).toBe(false);
		expect(get(value)).toEqual(1.23);

		apiClient.updateTreeDiameter = async (id: string, value: number): Promise<IResponse<ITree>> => {
			expect(id).toEqual('tree1');
			expect(value).toEqual(2.34);

			update_called = true;

			return {
				status: 200,
				data: TREE_RESPONSE
			};
		};

		value.set(2.34);

		await save();
		expect(update_called).toBe(true);

		expect(mockedGoto).toHaveBeenCalledWith('/map?preview=tree1');
	});
});
