import { goto } from '$app/navigation';
import { searchTrees } from '$lib/api/trees';
import { DEFAULT_MAP_CENTER, DEFAULT_TREE } from '$lib/constants';
import { routes } from '$lib/routes';
import { mapStore } from '$lib/stores/mapStore';
import { searchStore } from '$lib/stores/searchStore';
import { cleanup, render, screen, waitFor } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { get } from 'svelte/store';
import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest';
import Page from './+page.svelte';

let mockUrl = new URL('http://localhost/search/results?query=oak');

vi.mock('$app/state', () => ({
	get page() {
		return {
			url: mockUrl
		};
	}
}));

vi.mock('$app/navigation', () => ({
	goto: vi.fn()
}));

vi.mock('$lib/api/trees', () => ({
	searchTrees: vi.fn()
}));

const mockedGoto = vi.mocked(goto);
const mockedSearchTrees = vi.mocked(searchTrees);

describe('Search Results Page', () => {
	beforeEach(() => {
		mockedGoto.mockClear();
		mockedSearchTrees.mockReset();
		searchStore.set(undefined);
		mapStore.set({
			center: DEFAULT_MAP_CENTER,
			zoom: 15,
			bearing: 0
		});
		mockUrl = new URL('http://localhost/search/results?query=oak');
	});

	afterEach(() => {
		cleanup();
	});

	test('renders header, search results count, species list and preview links with details', async () => {
		mockedSearchTrees.mockResolvedValueOnce({
			status: 200,
			data: {
				trees: [
					{
						...DEFAULT_TREE,
						id: 'tree1',
						species: 'Quercus robur',
						state: 'healthy',
						height: 12,
						diameter: 8,
						circumference: 1.5
					},
					{
						...DEFAULT_TREE,
						id: 'tree2',
						species: 'Acer platanoides',
						state: 'dead',
						height: null,
						diameter: null,
						circumference: null
					}
				],
				users: []
			}
		});

		render(Page);

		expect(screen.getByText(/Search results/i)).toBeTruthy();
		expect(screen.queryByText('oak')).toBeNull();

		await waitFor(() => {
			expect(mockedSearchTrees).toHaveBeenCalledWith('oak', 15);
			expect(screen.getByText(/Search results \(2\)/i)).toBeTruthy();
			expect(get(searchStore)).toBe('oak');
		});

		const list = screen.getByRole('list');
		expect(list).toBeTruthy();
		expect(list.tagName.toLowerCase()).toBe('ul');

		const items = screen.getAllByRole('listitem');
		expect(items).toHaveLength(2);

		const tree1Link = screen.getByRole('link', { name: /Quercus robur alive/i });
		expect(tree1Link).toBeTruthy();
		expect(tree1Link.getAttribute('href')).toBe(routes.mapPreview('tree1'));
		expect(tree1Link.classList.contains('state-healthy')).toBe(true);
		expect(items[0].contains(tree1Link)).toBe(true);

		const tree1Primary = items[0].querySelector('.primary');
		expect(tree1Primary?.textContent?.trim()).toBe('Quercus robur alive');
		const tree1Secondary = items[0].querySelector('.secondary');
		expect(tree1Secondary?.textContent?.trim()).toBe('H=12 m D=8 m C=150 cm');

		const tree2Link = screen.getByRole('link', { name: /Acer platanoides dead/i });
		expect(tree2Link).toBeTruthy();
		expect(tree2Link.getAttribute('href')).toBe(routes.mapPreview('tree2'));
		expect(tree2Link.classList.contains('state-dead')).toBe(true);
		expect(items[1].contains(tree2Link)).toBe(true);

		const tree2Primary = items[1].querySelector('.primary');
		expect(tree2Primary?.textContent?.trim()).toBe('Acer platanoides dead');
		const tree2Secondary = items[1].querySelector('.secondary');
		expect(tree2Secondary?.textContent?.trim()).toBe('H=? D=? C=?');
	});

	test('applies state classes to tree links for all tree states', async () => {
		mockedSearchTrees.mockResolvedValueOnce({
			status: 200,
			data: {
				trees: [
					{ ...DEFAULT_TREE, id: 'tree-healthy', species: 'Tree Healthy', state: 'healthy' },
					{ ...DEFAULT_TREE, id: 'tree-alive', species: 'Tree Alive', state: 'alive' },
					{ ...DEFAULT_TREE, id: 'tree-dead', species: 'Tree Dead', state: 'dead' },
					{ ...DEFAULT_TREE, id: 'tree-stump', species: 'Tree Stump', state: 'stump' },
					{ ...DEFAULT_TREE, id: 'tree-gone', species: 'Tree Gone', state: 'gone' },
					{ ...DEFAULT_TREE, id: 'tree-replaced', species: 'Tree Replaced', state: 'replaced' }
				],
				users: []
			}
		});

		render(Page);

		await waitFor(() => {
			expect(screen.getByText(/Search results \(6\)/i)).toBeTruthy();
		});

		const treeHealthy = screen.getByRole('link', { name: /Tree Healthy/i });
		expect(treeHealthy.classList.contains('state-healthy')).toBe(true);

		const treeAlive = screen.getByRole('link', { name: /Tree Alive/i });
		expect(treeAlive.classList.contains('state-alive')).toBe(true);

		const treeDead = screen.getByRole('link', { name: /Tree Dead/i });
		expect(treeDead.classList.contains('state-dead')).toBe(true);

		const treeStump = screen.getByRole('link', { name: /Tree Stump/i });
		expect(treeStump.classList.contains('state-stump')).toBe(true);

		const treeGone = screen.getByRole('link', { name: /Tree Gone/i });
		expect(treeGone.classList.contains('state-gone')).toBe(true);

		const treeReplaced = screen.getByRole('link', { name: /Tree Replaced/i });
		expect(treeReplaced.classList.contains('state-replaced')).toBe(true);
	});

	test('filters out placeholder items from the results list', async () => {
		mockedSearchTrees.mockResolvedValueOnce({
			status: 200,
			data: {
				trees: [
					{ ...DEFAULT_TREE, id: 'tree1', species: 'Quercus robur', state: 'healthy' },
					{ ...DEFAULT_TREE, id: 'tree2', species: 'Placeholder tree', state: 'placeholder' }
				],
				users: []
			}
		});

		render(Page);

		await waitFor(() => {
			expect(mockedSearchTrees).toHaveBeenCalledWith('oak', 15);
			expect(screen.getByText(/Search results \(1\)/i)).toBeTruthy();
		});

		const list = screen.getByRole('list');
		expect(list).toBeTruthy();
		expect(list.tagName.toLowerCase()).toBe('ul');

		const items = screen.getAllByRole('listitem');
		expect(items).toHaveLength(1);

		const tree1Link = screen.getByRole('link', { name: /Quercus robur alive/i });
		expect(tree1Link).toBeTruthy();
		expect(items[0].contains(tree1Link)).toBe(true);
		expect(screen.queryByRole('link', { name: /Placeholder tree/i })).toBeNull();
	});

	test('displays no results when search returns empty list', async () => {
		mockedSearchTrees.mockResolvedValueOnce({
			status: 200,
			data: {
				trees: [],
				users: []
			}
		});

		render(Page);

		await waitFor(() => {
			expect(screen.getByText(/No trees found/i)).toBeTruthy();
		});
	});

	test('handles empty query parameter', async () => {
		mockUrl = new URL('http://localhost/search/results');

		render(Page);

		await waitFor(() => {
			expect(screen.getByText(/No search query provided/i)).toBeTruthy();
			expect(mockedSearchTrees).not.toHaveBeenCalled();
		});
	});

	test('navigates back when close button is clicked', async () => {
		const user = userEvent.setup();
		mockedSearchTrees.mockResolvedValueOnce({
			status: 200,
			data: {
				trees: [{ ...DEFAULT_TREE, id: 'tree1' }],
				users: []
			}
		});

		render(Page);

		const closeButton = screen.getByRole('button', { name: /close/i });
		await user.click(closeButton);

		await waitFor(() => {
			expect(mockedGoto).toHaveBeenCalledWith(routes.search());
			expect(get(searchStore)).toBeUndefined();
		});
	});

	test('handles API errors gracefully', async () => {
		mockedSearchTrees.mockResolvedValueOnce({
			status: 500,
			data: undefined,
			error: { code: 'SERVER_ERROR', description: 'Server error' }
		});

		render(Page);

		await waitFor(() => {
			expect(screen.getByText(/No trees found/i)).toBeTruthy();
		});
	});

	test('reloads tree search when map zoom changes', async () => {
		mockedSearchTrees.mockResolvedValue({
			status: 200,
			data: {
				trees: [{ ...DEFAULT_TREE, id: 'tree1', species: 'Quercus robur' }],
				users: []
			}
		});

		render(Page);

		await waitFor(() => {
			expect(mockedSearchTrees).toHaveBeenCalledWith('oak', 15);
		});

		mapStore.update((s) => ({ ...s, zoom: 18 }));

		await waitFor(() => {
			expect(mockedSearchTrees).toHaveBeenCalledWith('oak', 18);
		});
	});
});
