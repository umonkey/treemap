import { goto } from '$app/navigation';
import { routes } from '$lib/routes';
import { sidebarState } from '$lib/stores/sidebar.svelte';
import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest';
import { DialogState } from './Dialog.svelte.ts';

vi.mock('$app/navigation', () => ({
	goto: vi.fn()
}));

const mockedGoto = vi.mocked(goto);

describe('DialogState', () => {
	beforeEach(() => {
		mockedGoto.mockClear();
	});

	afterEach(() => {
		sidebarState.wide = false;
	});

	test('init widens the sidebar while open and resets it on cleanup', () => {
		const logic = new DialogState();

		const cleanup = logic.init();
		expect(sidebarState.wide).toBe(true);

		cleanup();
		expect(sidebarState.wide).toBe(false);
	});

	test('handleSearch navigates to the search results', async () => {
		const logic = new DialogState();

		await logic.handleSearch('oak');

		expect(mockedGoto).toHaveBeenCalledWith(routes.searchQuery('oak'));
	});

	test('handleInput stores the query', () => {
		const logic = new DialogState();

		logic.handleInput('oak');

		expect(logic.query).toBe('oak');
	});

	test('handleClose navigates home', async () => {
		const logic = new DialogState();

		await logic.handleClose();

		expect(mockedGoto).toHaveBeenCalledWith(routes.home());
	});
});
