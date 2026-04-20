import { goto } from '$app/navigation';
import { getTree, getTreeHistory, updateTreeState } from '$lib/api/trees';
import { DEFAULT_TREE } from '$lib/constants';
import { authStore } from '$lib/stores/authStore';
import { render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { describe, expect, test, vi } from 'vitest';
import DeleteTreeForm from './DeleteTreeForm.svelte';

vi.mock('$app/navigation', async () => {
	return {
		goto: vi.fn()
	};
});

vi.mock('$lib/api/trees', () => ({
	getTree: vi.fn(),
	updateTreeState: vi.fn(),
	getTreeHistory: vi.fn()
}));

const mockedGoto = vi.mocked(goto);

describe('DeleteTreeForm', async () => {
	test('handle form without comment', async () => {
		const user = userEvent.setup();

		let saved = false;
		let capturedComment: string | undefined;

		vi.mocked(getTree).mockResolvedValue({
			status: 200,
			data: {
				...DEFAULT_TREE,
				users: []
			}
		});

		vi.mocked(updateTreeState).mockImplementation(
			async (id: string, value: string | null, comment?: string) => {
				expect(id).toBe('tree1');
				expect(value).toBe('gone');
				capturedComment = comment;

				saved = true;

				return {
					status: 200,
					data: DEFAULT_TREE
				};
			}
		);

		authStore.set({
			token: 'secret',
			id: 'user1',
			name: 'John Doe',
			picture: 'https://example.com/picture.jpg'
		});

		vi.mocked(getTreeHistory).mockResolvedValue({
			status: 200,
			data: {
				props: [],
				users: []
			}
		});

		render(DeleteTreeForm, {
			id: 'tree1'
		});

		const confirm = (
			await screen.findAllByRole('button', {
				name: /save/i
			})
		)[0];

		await user.click(confirm);

		expect(saved).toBe(true);
		expect(capturedComment).toBeUndefined();
		expect(mockedGoto).toHaveBeenCalledWith('/tree/tree1/upload');
	});

	test('handle form with comment', async () => {
		const user = userEvent.setup();

		let saved = false;
		let capturedComment: string | undefined;

		vi.mocked(getTree).mockResolvedValue({
			status: 200,
			data: {
				...DEFAULT_TREE,
				users: []
			}
		});

		vi.mocked(updateTreeState).mockImplementation(
			async (id: string, value: string | null, comment?: string) => {
				expect(id).toBe('tree1');
				expect(value).toBe('gone');
				capturedComment = comment;

				saved = true;

				return {
					status: 200,
					data: DEFAULT_TREE
				};
			}
		);

		authStore.set({
			token: 'secret',
			id: 'user1',
			name: 'John Doe',
			picture: 'https://example.com/picture.jpg'
		});

		vi.mocked(getTreeHistory).mockResolvedValue({
			status: 200,
			data: {
				props: [],
				users: []
			}
		});

		render(DeleteTreeForm, {
			id: 'tree1'
		});

		const confirm = (
			await screen.findAllByRole('button', {
				name: /save/i
			})
		)[0];

		// Find the comment input and type a comment
		const commentInput = await screen.findByRole('textbox');
		await user.type(commentInput, 'duplicate of tree123');

		await user.click(confirm);

		expect(saved).toBe(true);
		expect(capturedComment).toBe('duplicate of tree123');
		expect(mockedGoto).toHaveBeenCalledWith('/tree/tree1/upload');
	});
});
