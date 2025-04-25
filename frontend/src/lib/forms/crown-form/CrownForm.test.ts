import { mockedGoto, TREE_RESPONSE, HISTORY_RESPONSE } from './mocks';

import CrownForm from './CrownForm.svelte';
import type { IResponse, ITree } from '$lib/types';
import userEvent from '@testing-library/user-event';
import { afterEach, describe, expect, test } from 'vitest';
import { apiClient } from '$lib/api';
import { cleanup, render, screen } from '@testing-library/svelte';
import { authStore } from '$lib/stores/authStore';

describe('CrownForm', async () => {
	afterEach(cleanup);

	test('should save changes', async () => {
		const user = userEvent.setup();

		let saved: boolean = false;

		apiClient.getTree = async () => {
			return {
				status: 200,
				data: TREE_RESPONSE
			};
		};

		apiClient.getTreeHistory = async (tree_id: string) => {
			expect(tree_id).toBe('tree1');

			return {
				status: 200,
				data: HISTORY_RESPONSE
			};
		};

		apiClient.updateTreeDiameter = async (id: string, value: number): Promise<IResponse<ITree>> => {
			expect(id).toBe('tree1');
			expect(value).toBe(2.34);

			saved = true;

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

		render(CrownForm, {
			id: 'tree1'
		});

		const input = await screen.findByRole('spinbutton');
		expect(input.value).toBe('1.23');
		await user.clear(input);
		await user.type(input, '2.34');

		const confirm = await screen.findByRole('button', {
			name: /save changes/i
		});

		await user.click(confirm);

		expect(saved).toBe(true);
		expect(mockedGoto).toHaveBeenCalledWith('/tree/tree1/history');
	});
});
