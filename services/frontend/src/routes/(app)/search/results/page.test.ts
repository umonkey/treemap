import { goto } from '$app/navigation';
import { searchTrees } from '$lib/api/trees';
import { mapBus } from '$lib/buses/mapBus';
import { DEFAULT_MAP_CENTER, DEFAULT_TREE } from '$lib/constants';
import { routes } from '$lib/routes';
import { mapStore } from '$lib/stores/mapStore';
import { searchStore } from '$lib/stores/searchStore';
import { cleanup, render, screen, waitFor } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { get } from 'svelte/store';
import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest';
import Page from './+page.svelte';
import { SearchResultsSidebarLogic } from './SearchResultsSidebar.svelte.ts';
const { bus } = vi.hoisted(() => {
	const listeners = new Map<string, Set<(event: unknown) => void>>();
	return {
		bus: {
			on(type: string, handler: (event: unknown) => void) {
				let set = listeners.get(type);
				if (!set) {
					set = new Set();
					listeners.set(type, set);
				}
				set.add(handler);
			},
			off(type: string, handler: (event: unknown) => void) {
				listeners.get(type)?.delete(handler);
			},
			emit(type: string, event: unknown) {
				const set = listeners.get(type);
				if (set) {
					for (const h of set) {
						h(event);
					}
				}
			}
		}
	};
});

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

vi.mock('$lib/buses/mapBus', () => ({
	mapBus: {
		emit: vi.fn((type, evt) => bus.emit(type, evt)),
		on: vi.fn((type, handler) => bus.on(type, handler)),
		off: vi.fn((type, handler) => bus.off(type, handler))
	}
}));

const mockedGoto = vi.mocked(goto);
const mockedSearchTrees = vi.mocked(searchTrees);
const mockedMapBusEmit = vi.mocked(mapBus.emit);
const mockedMapBusOn = vi.mocked(mapBus.on);
const mockedMapBusOff = vi.mocked(mapBus.off);

describe('Search Results Page', () => {
	beforeEach(() => {
		mockedGoto.mockClear();
		mockedMapBusEmit.mockClear();
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
		const user = userEvent.setup();
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
						circumference: 1.5,
						lat: 40.18,
						lon: 44.51
					},
					{
						...DEFAULT_TREE,
						id: 'tree2',
						species: 'Acer platanoides',
						state: 'dead',
						height: null,
						diameter: null,
						circumference: null,
						lat: 40.19,
						lon: 44.52
					}
				],
				users: []
			}
		});

		render(Page);

		expect(screen.getByText(/Search results/i)).toBeTruthy();
		expect(screen.queryByText('oak')).toBeNull();

		await waitFor(() => {
			expect(mockedSearchTrees).toHaveBeenCalledWith('oak', 15, undefined);
			expect(screen.getByText(/Search results \(2\)/i)).toBeTruthy();
			expect(get(searchStore)).toBe('oak');
		});

		const list = screen.getByRole('list');
		expect(list).toBeTruthy();
		expect(list.tagName.toLowerCase()).toBe('ul');

		const items = screen.getAllByRole('listitem');
		expect(items).toHaveLength(2);

		const tree1Card = screen.getByRole('button', { name: /Quercus robur alive/i });
		expect(tree1Card).toBeTruthy();
		expect(tree1Card.classList.contains('state-healthy')).toBe(true);
		expect(tree1Card.classList.contains('selected')).toBe(false);
		expect(tree1Card.getAttribute('aria-pressed')).toBe('false');
		expect(items[0].contains(tree1Card)).toBe(true);

		const tree1Primary = items[0].querySelector('.primary');
		expect(tree1Primary?.textContent?.trim()).toBe('Quercus robur alive');
		const tree1Secondary = items[0].querySelector('.secondary');
		expect(tree1Secondary?.textContent?.trim()).toBe('H=12 m D=8 m C=150 cm');

		const tree2Card = screen.getByRole('button', { name: /Acer platanoides dead/i });
		expect(tree2Card).toBeTruthy();
		expect(tree2Card.classList.contains('state-dead')).toBe(true);
		expect(tree2Card.classList.contains('selected')).toBe(false);
		expect(tree2Card.getAttribute('aria-pressed')).toBe('false');
		expect(items[1].contains(tree2Card)).toBe(true);

		const tree2Primary = items[1].querySelector('.primary');
		expect(tree2Primary?.textContent?.trim()).toBe('Acer platanoides dead');
		const tree2Secondary = items[1].querySelector('.secondary');
		expect(tree2Secondary?.textContent?.trim()).toBe('H=? D=? C=?');

		const previewButtons = screen.getAllByRole('button', { name: /Tree preview/i });
		expect(previewButtons).toHaveLength(2);

		// Clicking tree1 selects it
		await user.click(tree1Card);

		expect(mockedMapBusEmit).toHaveBeenCalledWith('move', { lat: 40.18, lng: 44.51 });
		expect(mockedMapBusEmit).toHaveBeenCalledWith('pin', { lat: 40.18, lng: 44.51 });
		expect(tree1Card.classList.contains('selected')).toBe(true);
		expect(tree1Card.getAttribute('aria-pressed')).toBe('true');
	});

	test('applies state classes to tree cards for all tree states', async () => {
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

		const treeHealthy = screen.getByRole('button', { name: /Tree Healthy/i });
		expect(treeHealthy.classList.contains('state-healthy')).toBe(true);

		const treeAlive = screen.getByRole('button', { name: /Tree Alive/i });
		expect(treeAlive.classList.contains('state-alive')).toBe(true);

		const treeDead = screen.getByRole('button', { name: /Tree Dead/i });
		expect(treeDead.classList.contains('state-dead')).toBe(true);

		const treeStump = screen.getByRole('button', { name: /Tree Stump/i });
		expect(treeStump.classList.contains('state-stump')).toBe(true);

		const treeGone = screen.getByRole('button', { name: /Tree Gone/i });
		expect(treeGone.classList.contains('state-gone')).toBe(true);

		const treeReplaced = screen.getByRole('button', { name: /Tree Replaced/i });
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
			expect(mockedSearchTrees).toHaveBeenCalledWith('oak', 15, undefined);
			expect(screen.getByText(/Search results \(1\)/i)).toBeTruthy();
		});

		const list = screen.getByRole('list');
		expect(list).toBeTruthy();
		expect(list.tagName.toLowerCase()).toBe('ul');

		const items = screen.getAllByRole('listitem');
		expect(items).toHaveLength(1);

		const tree1Card = screen.getByRole('button', { name: /Quercus robur alive/i });
		expect(tree1Card).toBeTruthy();
		expect(items[0].contains(tree1Card)).toBe(true);
		expect(screen.queryByRole('button', { name: /Placeholder tree/i })).toBeNull();
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

		const closeButton = screen.getByRole('button', { name: /^close$/i });
		await user.click(closeButton);

		await waitFor(() => {
			expect(mockedGoto).toHaveBeenCalledWith(routes.search());
			expect(get(searchStore)).toBeUndefined();
			expect(mockedMapBusEmit).toHaveBeenCalledWith('pin', undefined);
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
			expect(mockedSearchTrees).toHaveBeenCalledWith('oak', 15, undefined);
		});

		const newBounds = { n: 41, e: 45, s: 40, w: 44 };
		mapBus.emit('bounds', newBounds);

		await waitFor(() => {
			expect(mockedSearchTrees).toHaveBeenCalledWith('oak', 15, newBounds);
		});
	});

	test('emits move event with tree coordinates when a tree item is clicked', async () => {
		const user = userEvent.setup();
		mockedSearchTrees.mockResolvedValueOnce({
			status: 200,
			data: {
				trees: [
					{
						...DEFAULT_TREE,
						id: 'tree1',
						species: 'Quercus robur',
						state: 'healthy',
						lat: 40.18,
						lon: 44.51
					}
				],
				users: []
			}
		});

		render(Page);

		await waitFor(() => {
			expect(screen.getByText(/Search results \(1\)/i)).toBeTruthy();
		});

		const tree1Card = screen.getByRole('button', { name: /Quercus robur alive/i });
		await user.click(tree1Card);

		expect(mockedMapBusEmit).toHaveBeenCalledWith('move', { lat: 40.18, lng: 44.51 });
		expect(mockedMapBusEmit).toHaveBeenCalledWith('pin', { lat: 40.18, lng: 44.51 });
		expect(tree1Card.classList.contains('selected')).toBe(true);
		expect(tree1Card.getAttribute('aria-pressed')).toBe('true');

		const previewButton = screen.getByRole('button', { name: /Tree preview/i });
		expect(previewButton).toBeTruthy();
	});

	test('supports keyboard navigation with Enter and Space keys', async () => {
		const user = userEvent.setup();
		mockedSearchTrees.mockResolvedValueOnce({
			status: 200,
			data: {
				trees: [
					{
						...DEFAULT_TREE,
						id: 'tree1',
						species: 'Quercus robur',
						state: 'healthy',
						lat: 40.18,
						lon: 44.51
					},
					{
						...DEFAULT_TREE,
						id: 'tree2',
						species: 'Acer platanoides',
						state: 'dead',
						lat: 40.19,
						lon: 44.52
					}
				],
				users: []
			}
		});

		render(Page);

		await waitFor(() => {
			expect(screen.getByText(/Search results \(2\)/i)).toBeTruthy();
		});

		const tree1Card = screen.getByRole('button', { name: /Quercus robur alive/i });
		const tree2Card = screen.getByRole('button', { name: /Acer platanoides dead/i });

		tree1Card.focus();
		await user.keyboard('{Enter}');

		expect(mockedMapBusEmit).toHaveBeenCalledWith('move', { lat: 40.18, lng: 44.51 });
		expect(mockedMapBusEmit).toHaveBeenCalledWith('pin', { lat: 40.18, lng: 44.51 });
		expect(tree1Card.classList.contains('selected')).toBe(true);

		tree2Card.focus();
		await user.keyboard(' ');

		expect(mockedMapBusEmit).toHaveBeenCalledWith('move', { lat: 40.19, lng: 44.52 });
		expect(mockedMapBusEmit).toHaveBeenCalledWith('pin', { lat: 40.19, lng: 44.52 });
		expect(tree2Card.classList.contains('selected')).toBe(true);
		expect(tree1Card.classList.contains('selected')).toBe(false);
	});

	test('clicking preview button navigates to preview and stops event propagation', async () => {
		const user = userEvent.setup();
		mockedSearchTrees.mockResolvedValueOnce({
			status: 200,
			data: {
				trees: [
					{
						...DEFAULT_TREE,
						id: 'tree1',
						species: 'Quercus robur',
						state: 'healthy',
						lat: 40.18,
						lon: 44.51
					}
				],
				users: []
			}
		});

		render(Page);

		await waitFor(() => {
			expect(screen.getByText(/Search results \(1\)/i)).toBeTruthy();
		});

		mockedMapBusEmit.mockClear();

		const previewButton = screen.getByRole('button', { name: /Tree preview/i });
		expect(previewButton).toBeTruthy();

		await user.click(previewButton);

		expect(mockedGoto).toHaveBeenCalledWith(routes.mapPreview('tree1'));
		expect(mockedMapBusEmit).not.toHaveBeenCalled();
	});

	describe('SearchResultsSidebarLogic', () => {
		test('navigateToPreview prevents default, stops propagation, and navigates to tree preview', async () => {
			const logic = new SearchResultsSidebarLogic();
			const event = {
				preventDefault: vi.fn(),
				stopPropagation: vi.fn()
			} as unknown as Event;

			await logic.navigateToPreview(event, 'tree-123');

			expect(event.preventDefault).toHaveBeenCalled();
			expect(event.stopPropagation).toHaveBeenCalled();
			expect(mockedGoto).toHaveBeenCalledWith(routes.mapPreview('tree-123'));
		});

		test('selectTree sets selectedTreeId and emits map move', () => {
			const logic = new SearchResultsSidebarLogic();
			const tree = { ...DEFAULT_TREE, id: 'tree-123', lat: 40.18, lon: 44.51 };

			logic.selectTree(tree);

			expect(logic.selectedTreeId).toBe('tree-123');
			expect(mockedMapBusEmit).toHaveBeenCalledWith('move', { lat: 40.18, lng: 44.51 });
			expect(mockedMapBusEmit).toHaveBeenCalledWith('pin', { lat: 40.18, lng: 44.51 });
		});

		test('reload clears selectedTreeId on empty query', async () => {
			const logic = new SearchResultsSidebarLogic();
			logic.selectedTreeId = 'tree-123';

			await logic.reload('   ');

			expect(logic.selectedTreeId).toBeNull();
			expect(logic.trees).toEqual([]);
			expect(mockedMapBusEmit).toHaveBeenCalledWith('pin', undefined);
		});

		test('reload resets selectedTreeId if selected tree is not in new search results', async () => {
			const logic = new SearchResultsSidebarLogic();
			logic.selectedTreeId = 'tree-old';

			mockedSearchTrees.mockResolvedValueOnce({
				status: 200,
				data: {
					trees: [{ ...DEFAULT_TREE, id: 'tree-new', species: 'New tree' }],
					users: []
				}
			});

			await logic.reload('oak');

			expect(logic.selectedTreeId).toBeNull();
			expect(logic.trees).toHaveLength(1);
			expect(logic.trees[0].id).toBe('tree-new');
			expect(mockedMapBusEmit).toHaveBeenCalledWith('pin', undefined);
		});

		test('reload preserves selectedTreeId if selected tree is present in new results', async () => {
			const logic = new SearchResultsSidebarLogic();
			logic.selectedTreeId = 'tree-1';

			mockedSearchTrees.mockResolvedValueOnce({
				status: 200,
				data: {
					trees: [
						{ ...DEFAULT_TREE, id: 'tree-1', species: 'Tree 1' },
						{ ...DEFAULT_TREE, id: 'tree-2', species: 'Tree 2' }
					],
					users: []
				}
			});

			await logic.reload('oak');

			expect(logic.selectedTreeId).toBe('tree-1');
			expect(logic.trees).toHaveLength(2);
		});

		test('reload resets selectedTreeId on search error response and exception', async () => {
			const logic = new SearchResultsSidebarLogic();
			logic.selectedTreeId = 'tree-1';

			mockedSearchTrees.mockResolvedValueOnce({
				status: 500,
				data: undefined,
				error: { code: 'ERR', description: 'Error' }
			});

			await logic.reload('oak');

			expect(logic.selectedTreeId).toBeNull();
			expect(mockedMapBusEmit).toHaveBeenCalledWith('pin', undefined);

			logic.selectedTreeId = 'tree-2';
			mockedSearchTrees.mockRejectedValueOnce(new Error('Network failure'));

			await logic.reload('oak');

			expect(logic.selectedTreeId).toBeNull();
			expect(mockedMapBusEmit).toHaveBeenCalledWith('pin', undefined);
		});

		test('handleClose resets selectedTreeId and navigates to search', async () => {
			const logic = new SearchResultsSidebarLogic();
			logic.selectedTreeId = 'tree-1';

			await logic.handleClose();

			expect(logic.selectedTreeId).toBeNull();
			expect(mockedMapBusEmit).toHaveBeenCalledWith('pin', undefined);
			expect(mockedGoto).toHaveBeenCalledWith(routes.search());
		});

		test('init resets selectedTreeId initially and on cleanup', () => {
			const logic = new SearchResultsSidebarLogic();
			logic.selectedTreeId = 'tree-1';

			const cleanup = logic.init();
			expect(logic.selectedTreeId).toBeNull();

			logic.selectedTreeId = 'tree-2';
			cleanup();
			expect(logic.selectedTreeId).toBeNull();
			expect(mockedMapBusEmit).toHaveBeenCalledWith('pin', undefined);
		});

		test('initial search with unset bounds calculates result bounds and emits mapBus.fit', async () => {
			const logic = new SearchResultsSidebarLogic();
			mockedSearchTrees.mockResolvedValueOnce({
				status: 200,
				data: {
					trees: [
						{ ...DEFAULT_TREE, id: 't1', lat: 40.1, lon: 44.5 },
						{ ...DEFAULT_TREE, id: 't2', lat: 40.2, lon: 44.6 }
					],
					users: []
				}
			});

			await logic.reload('oak');

			expect(mockedSearchTrees).toHaveBeenCalledWith('oak', 15, undefined);
			expect(mockedMapBusEmit).toHaveBeenCalledWith('fit', {
				start: { lat: 40.1, lng: 44.5 },
				end: { lat: 40.2, lng: 44.6 }
			});
		});

		test('receiving mapBus bounds updates logic.bounds and reloads search trees with those bounds', async () => {
			const logic = new SearchResultsSidebarLogic();
			logic.query = 'oak';
			mockedSearchTrees.mockResolvedValue({
				status: 200,
				data: {
					trees: [{ ...DEFAULT_TREE, id: 't1', lat: 40.1, lon: 44.5 }],
					users: []
				}
			});

			logic.init();

			const newBounds = { n: 41, e: 45, s: 40, w: 44 };
			mapBus.emit('bounds', newBounds);

			expect(logic.bounds).toEqual(newBounds);
			expect(mockedSearchTrees).toHaveBeenCalledWith('oak', 15, newBounds);
		});

		test('cleanup unregisters mapBus.bounds listener', () => {
			const logic = new SearchResultsSidebarLogic();
			const cleanup = logic.init();

			expect(mockedMapBusOn).toHaveBeenCalledWith('bounds', logic.handleBounds);

			cleanup();

			expect(mockedMapBusOff).toHaveBeenCalledWith('bounds', logic.handleBounds);
		});
	});
});
