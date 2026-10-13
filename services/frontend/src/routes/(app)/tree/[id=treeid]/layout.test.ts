import { goto } from '$app/navigation';
import { routes } from '$lib/routes';
import { beforeEach, describe, expect, test, vi } from 'vitest';
import { TreeLayoutState } from './layout.svelte.ts';

vi.mock('$app/navigation', () => ({
	goto: vi.fn()
}));

const mockedGoto = vi.mocked(goto);

describe('TreeLayoutState', () => {
	beforeEach(() => {
		mockedGoto.mockClear();
	});

	test('handleClose(true) navigates home from the tree root', async () => {
		const logic = new TreeLayoutState();
		logic.id = '123';

		await logic.handleClose(true);

		expect(mockedGoto).toHaveBeenCalledWith(routes.home());
	});

	test('handleClose(false) navigates to the tree root from another tab', async () => {
		const logic = new TreeLayoutState();
		logic.id = '123';

		await logic.handleClose(false);

		expect(mockedGoto).toHaveBeenCalledWith(routes.treeDetails('123'));
	});

	test('handleClose does nothing without an id', async () => {
		const logic = new TreeLayoutState();

		await logic.handleClose(false);

		expect(mockedGoto).not.toHaveBeenCalled();
	});

	test('handleSearch navigates to the search results', async () => {
		const logic = new TreeLayoutState();

		await logic.handleSearch('oak');

		expect(mockedGoto).toHaveBeenCalledWith(routes.searchQuery('oak'));
	});

	test('handleInput stores the query', () => {
		const logic = new TreeLayoutState();

		logic.handleInput('oak');

		expect(logic.query).toBe('oak');
	});
});
