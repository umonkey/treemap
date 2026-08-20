import { goto } from '$app/navigation';
import { getGeoJSON } from '$lib/api/trees';
import { mapBus } from '$lib/buses/mapBus';
import { menuBus } from '$lib/buses/menuBus';
import { DEFAULT_MAP_CENTER } from '$lib/constants';
import { showError } from '$lib/errors';
import { routes } from '$lib/routes';
import { mapPoiStore } from '$lib/stores/mapPoi.svelte';
import { mapStore } from '$lib/stores/mapStore';
import { searchStore } from '$lib/stores/searchStore';
import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest';
import { TreeLayerState } from './TreeLayer.svelte.ts';

const { mapEventBus, menuEventBus } = vi.hoisted(() => {
	const createBus = () => {
		const listeners = new Map<string, Set<(event: unknown) => void>>();
		return {
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
			},
			clear() {
				listeners.clear();
			}
		};
	};
	return {
		mapEventBus: createBus(),
		menuEventBus: createBus()
	};
});

vi.mock('$app/navigation', () => ({
	goto: vi.fn()
}));

vi.mock('$lib/api/trees', () => ({
	getGeoJSON: vi.fn()
}));

vi.mock('$lib/errors', () => ({
	showError: vi.fn()
}));

vi.mock('$lib/buses/mapBus', () => ({
	mapBus: {
		emit: vi.fn((type, evt) => mapEventBus.emit(type, evt)),
		on: vi.fn((type, handler) => mapEventBus.on(type, handler)),
		off: vi.fn((type, handler) => mapEventBus.off(type, handler))
	}
}));

vi.mock('$lib/buses/menuBus', () => ({
	menuBus: {
		emit: vi.fn((type, evt) => menuEventBus.emit(type, evt)),
		on: vi.fn((type, handler) => menuEventBus.on(type, handler)),
		off: vi.fn((type, handler) => menuEventBus.off(type, handler))
	}
}));

const mockedGoto = vi.mocked(goto);
const mockedGetGeoJSON = vi.mocked(getGeoJSON);
const mockedShowError = vi.mocked(showError);
const mockedMapBusEmit = vi.mocked(mapBus.emit);
const mockedMapBusOn = vi.mocked(mapBus.on);
const mockedMapBusOff = vi.mocked(mapBus.off);
const mockedMenuBusEmit = vi.mocked(menuBus.emit);

const mockCollection = {
	type: 'FeatureCollection' as const,
	features: [
		{
			type: 'Feature' as const,
			id: 'tree-1',
			geometry: {
				type: 'Point',
				coordinates: [44.51, 40.18]
			},
			properties: {
				id: 'tree-1',
				state: 'healthy',
				type: 'tree',
				crown: 5,
				trunk: 20
			}
		}
	]
};

describe('TreeLayerState', () => {
	beforeEach(() => {
		vi.useFakeTimers();
		mockedGoto.mockClear();
		mockedGetGeoJSON.mockReset();
		mockedShowError.mockClear();
		mockedMapBusEmit.mockClear();
		mockedMapBusOn.mockClear();
		mockedMapBusOff.mockClear();
		mockedMenuBusEmit.mockClear();
		mapEventBus.clear();
		menuEventBus.clear();

		searchStore.set(undefined);
		mapStore.set({
			center: DEFAULT_MAP_CENTER,
			zoom: 15,
			bearing: 0
		});
		mapPoiStore.trees = [];

		if (!globalThis.navigator) {
			// @ts-expect-error test environment setup
			globalThis.navigator = {};
		}
		globalThis.navigator.vibrate = vi.fn();
	});

	afterEach(() => {
		vi.useRealTimers();
	});

	test('initial state has no markers and no bounds', () => {
		const state = new TreeLayerState();

		expect(state.markers).toBeUndefined();
		expect(state.bounds).toBeUndefined();
		expect(state.crownRadiusSmall).toBe(4);
		expect(state.crownRadiusLarge).toBeDefined();
		expect(state.trunkRadiusLarge).toBeDefined();
	});

	test('onMount registers listeners and cleanup unregisters them', () => {
		const state = new TreeLayerState();
		const cleanup = state.onMount();

		expect(mockedMapBusOn).toHaveBeenCalledWith('bounds', expect.any(Function));
		expect(mockedMapBusOn).toHaveBeenCalledWith('reload', expect.any(Function));

		state.bounds = { n: 40.2, s: 40.0, e: 44.6, w: 44.4 };

		cleanup();

		expect(state.bounds).toBeUndefined();
		expect(mockedMapBusOff).toHaveBeenCalledWith('bounds', expect.any(Function));
		expect(mockedMapBusOff).toHaveBeenCalledWith('reload', expect.any(Function));
	});

	test('receiving mapBus bounds updates bounds and triggers getGeoJSON with extended bounds, searchStore, and mapZoom', async () => {
		const state = new TreeLayerState();
		state.onMount();

		searchStore.set('oak');
		mapStore.set({
			center: DEFAULT_MAP_CENTER,
			zoom: 16,
			bearing: 0
		});

		mockedGetGeoJSON.mockResolvedValueOnce({
			status: 200,
			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			data: mockCollection as any
		});

		const inputBounds = { n: 40.5, s: 39.5, e: 44.5, w: 43.5 };
		mapBus.emit('bounds', inputBounds);

		expect(state.bounds).toEqual(inputBounds);

		await vi.advanceTimersByTimeAsync(150);

		// dLat = 1.0, dLon = 1.0 -> padding = 0.5
		// n: 41, e: 45, s: 39, w: 43
		expect(mockedGetGeoJSON).toHaveBeenCalledWith(41, 45, 39, 43, 'oak', 16);
		expect(state.markers).toEqual(mockCollection);
		expect(mapPoiStore.trees).toEqual([
			{
				lat: 40.18,
				lon: 44.51,
				url: routes.mapPreview('tree-1')
			}
		]);
	});

	test('receiving mapBus reload reloads data when bounds are present', async () => {
		const state = new TreeLayerState();
		state.onMount();

		// When bounds are not set, reload does nothing
		mapBus.emit('reload');
		await vi.advanceTimersByTimeAsync(150);
		expect(mockedGetGeoJSON).not.toHaveBeenCalled();

		// Now set bounds
		mockedGetGeoJSON.mockResolvedValue({
			status: 200,
			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			data: mockCollection as any
		});

		mapBus.emit('bounds', { n: 40.2, s: 40.0, e: 44.6, w: 44.4 });
		await vi.advanceTimersByTimeAsync(150);
		expect(mockedGetGeoJSON).toHaveBeenCalledTimes(1);

		mockedGetGeoJSON.mockClear();

		// Trigger reload
		mapBus.emit('reload');
		await vi.advanceTimersByTimeAsync(150);
		expect(mockedGetGeoJSON).toHaveBeenCalledTimes(1);
	});

	test('handles getGeoJSON error and shows error toast', async () => {
		const state = new TreeLayerState();
		state.onMount();

		mockedGetGeoJSON.mockRejectedValueOnce(new Error('Network error'));

		mapBus.emit('bounds', { n: 40.2, s: 40.0, e: 44.6, w: 44.4 });
		await vi.advanceTimersByTimeAsync(150);

		expect(mockedShowError).toHaveBeenCalledWith('Error loading trees, please try again.');
	});

	test('handleClick ignores click when mapZoom < 15', async () => {
		const state = new TreeLayerState();
		mapStore.set({
			center: DEFAULT_MAP_CENTER,
			zoom: 14,
			bearing: 0
		});

		const event = {
			features: [
				{
					properties: { id: 'tree-123' },
					geometry: { coordinates: [44.51, 40.18] }
				}
			]
		};

		await state.handleClick(event);

		expect(mockedMapBusEmit).not.toHaveBeenCalledWith('move', expect.anything());
		expect(mockedGoto).not.toHaveBeenCalled();
	});

	test('handleClick does nothing when features are empty or missing', async () => {
		const state = new TreeLayerState();
		mapStore.set({
			center: DEFAULT_MAP_CENTER,
			zoom: 16,
			bearing: 0
		});

		await state.handleClick({ features: [] });
		await state.handleClick({});

		expect(mockedMapBusEmit).not.toHaveBeenCalledWith('move', expect.anything());
		expect(mockedGoto).not.toHaveBeenCalled();
	});

	test('handleClick emits move, navigates to preview, and vibrates when zoom >= 15', async () => {
		const state = new TreeLayerState();
		mapStore.set({
			center: DEFAULT_MAP_CENTER,
			zoom: 16,
			bearing: 0
		});

		const event = {
			features: [
				{
					properties: { id: 'tree-123' },
					geometry: { coordinates: [44.51, 40.18] }
				}
			]
		};

		await state.handleClick(event);

		expect(mockedMapBusEmit).toHaveBeenCalledWith('move', { lat: 40.18, lng: 44.51 });
		expect(mockedGoto).toHaveBeenCalledWith(routes.mapPreview('tree-123'));
		expect(navigator.vibrate).toHaveBeenCalledWith(50);
	});

	test('handleContextMenu ignores event when mapZoom < 15', () => {
		const state = new TreeLayerState();
		mapStore.set({
			center: DEFAULT_MAP_CENTER,
			zoom: 14,
			bearing: 0
		});

		const event = {
			features: [
				{
					properties: { id: 'tree-123' },
					geometry: { coordinates: [44.51, 40.18] }
				}
			]
		};

		state.handleContextMenu(event);

		expect(mockedMenuBusEmit).not.toHaveBeenCalled();
	});

	test('handleContextMenu does nothing when features are empty or missing', () => {
		const state = new TreeLayerState();
		mapStore.set({
			center: DEFAULT_MAP_CENTER,
			zoom: 16,
			bearing: 0
		});

		state.handleContextMenu({ features: [] });
		state.handleContextMenu({});

		expect(mockedMenuBusEmit).not.toHaveBeenCalled();
	});

	test('handleContextMenu emits menuBus show and vibrates when zoom >= 15', () => {
		const state = new TreeLayerState();
		mapStore.set({
			center: DEFAULT_MAP_CENTER,
			zoom: 16,
			bearing: 0
		});

		const event = {
			features: [
				{
					properties: { id: 'tree-456' },
					geometry: { coordinates: [44.51, 40.18] }
				}
			]
		};

		state.handleContextMenu(event);

		expect(mockedMenuBusEmit).toHaveBeenCalledWith('show', 'tree-456');
		expect(navigator.vibrate).toHaveBeenCalledWith(50);
	});
});
