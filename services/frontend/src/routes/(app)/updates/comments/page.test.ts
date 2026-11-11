import { getRecentComments } from '$lib/api/comments';
import { addTrees } from '$lib/stores/treeStore';
import { addUsers } from '$lib/stores/userStore';
import type { IComment } from '$lib/types';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { PageState } from './page.svelte.ts';

vi.mock('$lib/api/comments', () => ({
	getRecentComments: vi.fn()
}));

vi.mock('$lib/stores/userStore', () => ({
	addUsers: vi.fn()
}));

vi.mock('$lib/stores/treeStore', () => ({
	addTrees: vi.fn()
}));

const mockedGetRecentComments = vi.mocked(getRecentComments);
const mockedAddUsers = vi.mocked(addUsers);
const mockedAddTrees = vi.mocked(addTrees);

const comment = (overrides: Partial<IComment> = {}): IComment => ({
	id: '1',
	entity_type: 'tree',
	entity_id: '10',
	added_at: 0,
	added_by: 'user1',
	message: 'Hello',
	...overrides
});

describe('updates comments PageState', () => {
	beforeEach(() => {
		mockedGetRecentComments.mockReset();
		mockedAddUsers.mockReset();
		mockedAddTrees.mockReset();
	});

	it('keeps only tree comments sorted newest first', async () => {
		const olderTree = comment({ id: '1', added_at: 10 });
		const newerTree = comment({ id: '2', added_at: 20 });
		const panorama = comment({ id: '3', added_at: 30, entity_type: 'panorama' });

		mockedGetRecentComments.mockResolvedValueOnce({
			status: 200,
			data: { comments: [olderTree, panorama, newerTree], users: [], trees: [] }
		});

		const state = new PageState();

		await state.load();

		expect(mockedAddUsers).toHaveBeenCalledWith([]);
		expect(mockedAddTrees).toHaveBeenCalledWith([]);
		expect(state.comments).toEqual([newerTree, olderTree]);
		expect(state.error).toBe(false);
	});
});
