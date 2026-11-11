import { getComments } from '$lib/api/comments';
import { addUsers } from '$lib/stores/userStore';
import type { IComment } from '$lib/types';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { CommentThreadState } from './CommentThread.svelte.ts';

vi.mock('$lib/api/comments', () => ({
	getComments: vi.fn()
}));

vi.mock('$lib/stores/userStore', () => ({
	addUsers: vi.fn()
}));

const mockedGetComments = vi.mocked(getComments);
const mockedAddUsers = vi.mocked(addUsers);

const comment = (overrides: Partial<IComment> = {}): IComment => ({
	id: '1',
	entity_type: 'panorama',
	entity_id: '42',
	added_at: 0,
	added_by: 'user1',
	message: 'Hello',
	...overrides
});

describe('CommentThreadState', () => {
	beforeEach(() => {
		mockedGetComments.mockReset();
		mockedAddUsers.mockReset();
	});

	it('loads the comments of an entity sorted newest first', async () => {
		const older = comment({ id: '1', added_at: 10 });
		const newer = comment({ id: '2', added_at: 20 });

		mockedGetComments.mockResolvedValueOnce({
			status: 200,
			data: { comments: [older, newer], users: [], trees: [] }
		});

		const state = new CommentThreadState();

		await state.reload('panorama', '42');

		expect(mockedGetComments).toHaveBeenCalledWith('panorama', '42');
		expect(mockedAddUsers).toHaveBeenCalledWith([]);
		expect(state.comments).toEqual([newer, older]);
	});

	it('keeps authorless system comments unchanged', async () => {
		const system = comment({ id: '3', added_by: null, message: 'Processing failed: boom' });

		mockedGetComments.mockResolvedValueOnce({
			status: 200,
			data: { comments: [system], users: [], trees: [] }
		});

		const state = new CommentThreadState();

		await state.reload('panorama', '42');

		expect(state.comments).toEqual([system]);
	});

	it('leaves the comments empty when loading fails', async () => {
		mockedGetComments.mockResolvedValueOnce({
			status: 500,
			error: { code: 'Internal', description: 'Something went wrong.' }
		});

		const state = new CommentThreadState();

		await state.reload('panorama', '42');

		expect(state.comments).toEqual([]);
	});
});
