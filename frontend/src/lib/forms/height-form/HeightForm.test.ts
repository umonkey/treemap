import HeightForm from './HeightForm.svelte';
import type { IResponse, ISingleTree, ITree, IChangeList } from '$lib/types';
import userEvent from '@testing-library/user-event';
import { DEFAULT_TREE } from '$lib/constants';
import { afterEach, describe, expect, test, vi } from 'vitest';
import { apiClient } from '$lib/api';
import { cleanup, render, screen } from '@testing-library/svelte';
import { goto } from '$app/navigation';
import { authStore } from '$lib/stores/authStore';

const TREE_RESPONSE = {
	...DEFAULT_TREE,
	id: 'tree1',
	users: []
} as ISingleTree;

const HISTORY_RESPONSE = {
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

const mockedGoto = vi.mocked(goto);

describe('HeightForm', async () => {
	afterEach(cleanup);

	test('should save changes', async () => {
		const user = userEvent.setup();

		let saved: boolean = false;

		apiClient.getTree = async (): Promise<IResponse<ISingleTree>> => {
			return {
				status: 200,
				data: TREE_RESPONSE
			};
		};

		apiClient.getTreeHistory = async (): Promise<IResponse<IChangeList>> => {
			return {
				status: 200,
				data: HISTORY_RESPONSE
			};
		};

		apiClient.updateTreeHeight = async (id: string, value: number): Promise<IResponse<ITree>> => {
			expect(id).toBe('tree1');
			expect(value).toBe(1.23);

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

		render(HeightForm, {
			id: 'tree1'
		});

		const input = await screen.findByRole('spinbutton');
		await user.type(input, '1.23');

		const confirm = await screen.findByRole('button', {
			name: /save changes/i
		});

		await user.click(confirm);

		expect(saved).toBe(true);
		expect(mockedGoto).toHaveBeenCalledWith('/tree/tree1/history');
	});
});
