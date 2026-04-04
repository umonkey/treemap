// This must go first for the mocks to work.
import { TREE_RESPONSE, mockedGoto } from './mocks';

import * as treesApi from '$lib/api/trees';
import { authStore } from '$lib/stores/authStore';
import type { IResponse, ITree } from '$lib/types';
import { cleanup, render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { afterEach, describe, expect, test, vi } from 'vitest';
import MoveForm from './MoveForm.svelte';

vi.mock('$lib/api/trees', () => ({
	getTree: vi.fn(),
	updateTreeLocation: vi.fn(),
	getTreeHistory: vi.fn()
}));

describe('MoveForm', async () => {
	afterEach(cleanup);

	test('should save changes', async () => {
		const user = userEvent.setup();

		let update_called = false;

		vi.mocked(treesApi.getTree).mockResolvedValue({
			status: 200,
			data: TREE_RESPONSE
		});

		vi.mocked(treesApi.getTreeHistory).mockResolvedValue({
			status: 200,
			data: { props: [], users: [] }
		});

		vi.mocked(treesApi.updateTreeLocation).mockImplementation(
			async (id: string, lat: number, lon: number): Promise<IResponse<ITree>> => {
				expect(id).toBe('tree1');
				expect(lat).toBe(1.23);
				expect(lon).toBe(2.34);

				update_called = true;

				return {
					status: 200,
					data: TREE_RESPONSE
				};
			}
		);

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
		expect(mockedGoto).toHaveBeenCalledWith('/?preview=tree1');
	});
});
