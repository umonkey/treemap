import { goto } from '$app/navigation';
import { updateTreeDiameter } from '$lib/api/trees';
import { DEFAULT_TREE } from '$lib/constants';
import type { IResponse, ITree } from '$lib/types';
import { get } from 'svelte/store';
import { describe, expect, it, vi } from 'vitest';
import { updateCrownDiameter } from './updateCrownDiameter';

vi.mock('$app/navigation', async () => {
	return {
		goto: vi.fn()
	};
});

vi.mock('$lib/api/trees', () => ({
	updateTreeDiameter: vi.fn()
}));

const mockedGoto = vi.mocked(goto);

describe('actions/updateCrownDiameter', async () => {
	it('should call the API', async () => {
		let called = false;

		vi.mocked(updateTreeDiameter).mockImplementation(
			async (id: string, value: number): Promise<IResponse<ITree>> => {
				expect(id).toEqual('tree1');
				expect(value).toEqual(1.23);

				called = true;

				return {
					status: 200,
					data: DEFAULT_TREE
				};
			}
		);

		const { busy, handleConfirm } = updateCrownDiameter('tree1');
		expect(get(busy)).toBe(false);

		await handleConfirm(1.23);

		expect(get(busy)).toBe(false);
		expect(called).toBe(true);

		expect(mockedGoto).toHaveBeenCalledWith('/map?preview=tree1');
	});
});
