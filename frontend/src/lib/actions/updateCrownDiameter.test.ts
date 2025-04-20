import type { ITree, IResponse } from '$lib/types';
import { DEFAULT_TREE } from '$lib/constants';
import { apiClient } from '$lib/api';
import { describe, expect, it, vi } from 'vitest';
import { get } from 'svelte/store';
import { goto } from '$app/navigation';
import { updateCrownDiameter } from './updateCrownDiameter';

vi.mock('$app/navigation', async () => {
	return {
		goto: vi.fn()
	};
});

const mockedGoto = vi.mocked(goto);

describe('actions/updateCrownDiameter', async () => {
	it('should call the API', async () => {
		let called: boolean = false;

		apiClient.updateTreeDiameter = async (id: string, value: number): Promise<IResponse<ITree>> => {
			expect(id).toEqual('tree1');
			expect(value).toEqual(1.23);

			called = true;

			return {
				status: 200,
				data: DEFAULT_TREE
			};
		};

		const { busy, handleConfirm } = updateCrownDiameter('tree1');
		expect(get(busy)).toBe(false);

		await handleConfirm(1.23);

		expect(get(busy)).toBe(false);
		expect(called).toBe(true);

		expect(mockedGoto).toHaveBeenCalledWith('/tree/tree1');
	});
});
