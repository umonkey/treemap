import type { ITree, IResponse } from '$lib/types';
import { DEFAULT_TREE } from '$lib/constants';
import { apiClient } from '$lib/api';
import { describe, expect, it, vi } from 'vitest';
import { get } from 'svelte/store';
import { goto } from '$app/navigation';
import { markDead } from './markDead';

vi.mock('$app/navigation', async () => {
	return {
		goto: vi.fn()
	};
});

const mockedGoto = vi.mocked(goto);

describe('actions/markDead', async () => {
	it('should call the API', async () => {
		let called: boolean = false;

		apiClient.updateTreeState = async (id: string, state: string): Promise<IResponse<ITree>> => {
			expect(id).toEqual('tree1');
			expect(state).toEqual('dead');

			called = true;

			return {
				status: 200,
				data: DEFAULT_TREE
			};
		};

		const { busy, handleConfirm } = markDead('tree1');
		expect(get(busy)).toBe(false);

		await handleConfirm();

		expect(get(busy)).toBe(false);
		expect(called).toBe(true);

		expect(mockedGoto).toHaveBeenCalledWith('/map?preview=tree1');
	});
});
