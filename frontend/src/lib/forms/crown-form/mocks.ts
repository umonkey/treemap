import { goto } from '$app/navigation';
import { vi } from 'vitest';
import type { ISingleTree, IChangeList } from '$lib/types';
import { DEFAULT_TREE } from '$lib/constants';

export const TREE_RESPONSE = {
	...DEFAULT_TREE,
	id: 'tree1',
	diameter: 1.23
} as ISingleTree;

export const HISTORY_RESPONSE = {
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

export const mockedGoto = vi.mocked(goto);
