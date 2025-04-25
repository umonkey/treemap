import TreeDeadForm from './TreeDeadForm.svelte';
import type { IResponse, ISingleTree, ITree } from '$lib/types';
import userEvent from '@testing-library/user-event';
import { DEFAULT_TREE } from '$lib/constants';
import { afterEach, describe, expect, test, vi } from 'vitest';
import { apiClient } from '$lib/api';
import { cleanup, render, screen } from '@testing-library/svelte';
import { goto } from '$app/navigation';
import { authStore } from '$lib/stores/authStore';

vi.mock('$app/navigation', async () => {
	return {
		goto: vi.fn()
	};
});

const mockedGoto = vi.mocked(goto);

describe('TreeDeadForm', async () => {
	afterEach(cleanup);

	test('handle form', async () => {
		const user = userEvent.setup();

		let saved: boolean = false;

		apiClient.getTree = async (): Promise<IResponse<ISingleTree>> => {
			console.debug('GET');

			return {
				status: 200,
				data: {
					...DEFAULT_TREE,
					users: []
				}
			};
		};

		apiClient.updateTreeState = async (id: string, value: string): Promise<IResponse<ITree>> => {
			expect(id).toBe('tree1');
			expect(value).toBe('dead');

			saved = true;

			return {
				status: 200,
				data: DEFAULT_TREE
			};
		};

		authStore.set({
			token: 'secret',
			name: 'John Doe',
			picture: 'https://example.com/picture.jpg'
		});

		render(TreeDeadForm, {
			id: 'tree1'
		});

		const confirm = await screen.findByRole('button', {
			name: /confirm/i
		});

		await user.click(confirm);

		expect(saved).toBe(true);
		expect(mockedGoto).toHaveBeenCalledWith('/tree/tree1/history');
	});
});
