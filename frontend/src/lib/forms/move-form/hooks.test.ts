// This must go first for the mocks to work.
import { mockedGoto, TREE_RESPONSE, HISTORY_RESPONSE } from './mocks';

import type { ITree, IResponse, ISingleTree } from '$lib/types';
import { apiClient } from '$lib/api';
import { describe, expect, it } from 'vitest';
import { get } from 'svelte/store';
import { editor } from './hooks';

describe('move-form/hooks', async () => {
	it('should call the update API', async () => {
		const tree_id = 'tree1';

		let update_called = false;

		apiClient.getTree = async (): Promise<IResponse<ISingleTree>> => ({
			status: 200,
			data: TREE_RESPONSE
		});

		apiClient.getTreeHistory = async () => ({
			status: 200,
			data: HISTORY_RESPONSE
		});

		const { loading, value, history, reload, save } = editor(tree_id);

		expect(get(loading)).toBe(true);

		expect(get(value)).toStrictEqual({
			lat: 0,
			lng: 0
		});

		expect(get(history)).toHaveLength(0);

		await reload(tree_id);
		expect(get(loading)).toBe(false);

		expect(get(value)).toEqual({
			lat: 1.23,
			lng: 2.34
		});

		expect(get(history)).toHaveLength(1);

		apiClient.updateTreeLocation = async (
			id: string,
			lat: number,
			lon: number
		): Promise<IResponse<ITree>> => {
			expect(id).toEqual(tree_id);
			expect(lat).toEqual(1.23);
			expect(lon).toEqual(2.34);

			update_called = true;

			return {
				status: 200,
				data: TREE_RESPONSE
			};
		};

		await save();
		expect(update_called).toBe(true);

		expect(mockedGoto).toHaveBeenCalledWith('/tree/tree1');
	});
});
