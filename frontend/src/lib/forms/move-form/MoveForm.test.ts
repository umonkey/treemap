// This must go first for the mocks to work.
import { mockedGoto, TREE_RESPONSE, HISTORY_RESPONSE } from './mocks';

import MoveForm from './MoveForm.svelte';
import type { IResponse, ISingleTree, ITree, IChangeList } from '$lib/types';
import userEvent from '@testing-library/user-event';
import { afterEach, describe, expect, test } from 'vitest';
import { apiClient } from '$lib/api';
import { cleanup, render, screen } from '@testing-library/svelte';
import { authStore } from '$lib/stores/authStore';

describe('MoveForm', async () => {
	afterEach(cleanup);

	test('should save changes', async () => {
		const user = userEvent.setup();

		let update_called: boolean = false;

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

		apiClient.updateTreeLocation = async (
			id: string,
			lat: number,
			lon: number
		): Promise<IResponse<ITree>> => {
			expect(id).toBe('tree1');
			expect(lat).toBe(1.23);
			expect(lon).toBe(2.34);

			update_called = true;

			return {
				status: 200,
				data: TREE_RESPONSE
			};
		};

		authStore.set({
			token: 'secret',
			name: 'John Doe',
			picture: 'https://example.com/picture.jpg'
		});

		render(MoveForm, {
			id: 'tree1'
		});

		const confirm = await screen.findByRole('button', {
			name: /save changes/i
		});

		await user.click(confirm);

		expect(update_called).toBe(true);
		expect(mockedGoto).toHaveBeenCalledWith('/map?preview=tree1');
	});
});
