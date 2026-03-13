import { TREE_RESPONSE, mockedGoto } from './mocks';

import { apiClient } from '$lib/api';
import { authStore } from '$lib/stores/authStore';
import type { IResponse, ITree } from '$lib/types';
import { cleanup, render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { afterEach, describe, expect, test } from 'vitest';
import CircumferenceForm from './CircumferenceForm.svelte';

describe('CircumferenceForm', async () => {
	afterEach(cleanup);

	test('should save changes', async () => {
		const user = userEvent.setup();

		let saved = false;

		apiClient.getTree = async () => {
			return {
				status: 200,
				data: TREE_RESPONSE
			};
		};

		apiClient.updateTreeCircumference = async (
			id: string,
			value: number
		): Promise<IResponse<ITree>> => {
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

		render(CircumferenceForm, {
			id: 'tree1'
		});

		const input = (await screen.findByRole('spinbutton')) as HTMLInputElement;
		expect(input.value).toBe('123'); // NB: the control converts m to cm!
		await user.clear(input);
		await user.type(input, '234');

		const confirm = await screen.findByRole('button', {
			name: /submit changes/i
		});

		await user.click(confirm);

		expect(saved).toBe(true);
		expect(mockedGoto).toHaveBeenCalledWith('/map?preview=tree1');
	});
});
