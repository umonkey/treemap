import { goto } from '$app/navigation';
import { getTree, getTreeHistory, updateTreeState } from '$lib/api/trees';
import { DEFAULT_TREE } from '$lib/constants';
import { authStore } from '$lib/stores/authStore';
import type { IChangeList, IResponse, ISingleTree, ITree } from '$lib/types';
import { cleanup, render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { afterEach, describe, expect, test, vi } from 'vitest';
import TreeDeadForm from './TreeDeadForm.svelte';

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

describe('TreeDeadForm', async () => {
	afterEach(cleanup);

	test('handle form', async () => {
		const user = userEvent.setup();

		let saved = false;

		vi.mocked(getTree).mockImplementation(async (): Promise<IResponse<ISingleTree>> => {
			console.debug('GET');

			return {
				status: 200,
				data: {
					...DEFAULT_TREE,
					users: []
				}
			};
		});

		vi.mocked(updateTreeState).mockImplementation(
			async (id: string, value: string | null): Promise<IResponse<ITree>> => {
				expect(id).toBe('tree1');
				expect(value).toBe('dead');

				saved = true;

				return {
					status: 200,
					data: DEFAULT_TREE
				};
			}
		);

		authStore.set({
			token: 'secret',
			name: 'John Doe',
			picture: 'https://example.com/picture.jpg'
		});

		vi.mocked(getTreeHistory).mockImplementation(async (): Promise<IResponse<IChangeList>> => {
			return {
				status: 200,
				data: {
					props: [],
					users: []
				}
			};
		});

		render(TreeDeadForm, {
			id: 'tree1'
		});

		const confirm = (
			await screen.findAllByRole('button', {
				name: /save/i
			})
		)[0];

		await user.click(confirm);

		expect(saved).toBe(true);
		expect(mockedGoto).toHaveBeenCalledWith('/tree/tree1/upload');
	});
});
