import { goto } from '$app/navigation';
import { apiClient } from '$lib/api';
import { DEFAULT_TREE } from '$lib/constants';
import { authStore } from '$lib/stores/authStore';
import type { IChangeList, IResponse, ISingleTree, ITree } from '$lib/types';
import { cleanup, render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { afterEach, describe, expect, test, vi } from 'vitest';
import HeightForm from './HeightForm.svelte';

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

		let saved = false;

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
			name: /submit changes/i
		});

		await user.click(confirm);

		expect(saved).toBe(true);
		expect(mockedGoto).toHaveBeenCalledWith('/map?preview=tree1');
	});
});
