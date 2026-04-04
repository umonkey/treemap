import { goto } from '$app/navigation';
import { updateTreeState } from '$lib/api/trees';
import { DEFAULT_TREE } from '$lib/constants';
import type { IResponse, ITree } from '$lib/types';
import { get } from 'svelte/store';
import { describe, expect, it, vi } from 'vitest';
import { markDead } from './markDead';

vi.mock('$app/navigation', async () => {
	return {
		goto: vi.fn()
	};
});

vi.mock('$lib/api/trees', () => ({
	updateTreeState: vi.fn()
}));

const mockedGoto = vi.mocked(goto);

describe('actions/markDead', async () => {
	it('should call the API', async () => {
		let called = false;

		vi.mocked(updateTreeState).mockImplementation(
			async (id: string, value: string | null): Promise<IResponse<ITree>> => {
				expect(id).toEqual('tree1');
				expect(value).toEqual('dead');

				called = true;

				return {
					status: 200,
					data: DEFAULT_TREE
				};
			}
		);

		const { busy, handleConfirm } = markDead('tree1');
		expect(get(busy)).toBe(false);

		await handleConfirm();

		expect(get(busy)).toBe(false);
		expect(called).toBe(true);

		expect(mockedGoto).toHaveBeenCalledWith('/map?preview=tree1');
	});
});
