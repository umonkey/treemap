import { goto } from '$app/navigation';
import { vi } from 'vitest';
import type { ISingleTree, IChangeList } from '$lib/types';
import { DEFAULT_TREE } from '$lib/constants';

export const TREE_RESPONSE = {
	...DEFAULT_TREE,
	id: 'tree1',
	lat: 1.23,
	lon: 2.34
} as ISingleTree;

export const HISTORY_RESPONSE = {
	props: [
		{
			id: 'change1',
			tree_id: 'tree1',
			name: 'location',
			value: '1,2',
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

export const mockedGoto = vi.mocked(goto);
