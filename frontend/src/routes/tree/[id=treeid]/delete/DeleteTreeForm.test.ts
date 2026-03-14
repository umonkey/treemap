import { goto } from '$app/navigation';
import { apiClient } from '$lib/api';
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

const mockedGoto = vi.mocked(goto);

describe('DeleteTreeForm', async () => {
	test('handle form without comment', async () => {
		const user = userEvent.setup();

		let saved = false;
		let capturedComment: string | undefined;

		apiClient.getTree = vi.fn().mockResolvedValue({
			status: 200,
			data: {
				...DEFAULT_TREE,
				users: []
			}
		});

		apiClient.updateTreeState = vi
			.fn()
			.mockImplementation(async (id: string, value: string, comment?: string) => {
				expect(id).toBe('tree1');
				expect(value).toBe('gone');
				capturedComment = comment;

				saved = true;

				return {
					status: 200,
					data: DEFAULT_TREE
				};
			});

		authStore.set({
			token: 'secret',
			name: 'John Doe',
			picture: 'https://example.com/picture.jpg'
		});

		apiClient.getTreeHistory = vi.fn().mockResolvedValue({
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

		apiClient.getTree = vi.fn().mockResolvedValue({
			status: 200,
			data: {
				...DEFAULT_TREE,
				users: []
			}
		});

		apiClient.updateTreeState = vi
			.fn()
			.mockImplementation(async (id: string, value: string, comment?: string) => {
				expect(id).toBe('tree1');
				expect(value).toBe('gone');
				capturedComment = comment;

				saved = true;

				return {
					status: 200,
					data: DEFAULT_TREE
				};
			});

		authStore.set({
			token: 'secret',
			name: 'John Doe',
			picture: 'https://example.com/picture.jpg'
		});

		apiClient.getTreeHistory = vi.fn().mockResolvedValue({
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
