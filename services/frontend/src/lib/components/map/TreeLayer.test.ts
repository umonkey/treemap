import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { TreeLayerLogic } from './TreeLayer.svelte.ts';
import { mapBus } from '$lib/buses/mapBus';
import { menuBus } from '$lib/buses/menuBus';
import { mapPoiStore } from '$lib/stores/mapPoi.svelte';
import { getGeoJSON } from '$lib/api/trees';
import * as routesModule from '$lib/routes';
import type { IBounds, IMarkers } from '$lib/types';

vi.mock('$lib/api/trees', () => ({
	getGeoJSON: vi.fn()
}));

vi.mock('$lib/routes', async () => {
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	const actual = (await vi.importActual('$lib/routes')) as any;
	return {
		...actual,
		goto: vi.fn().mockResolvedValue(undefined),
		routes: {
			...actual.routes,
			mapPreview: (id: string) => `/tree/${id}`
		}
	};
});

describe('TreeLayerLogic', () => {
	let logic: TreeLayerLogic;

	beforeEach(() => {
		vi.clearAllMocks();
		mapPoiStore.trees = [];
		vi.mocked(getGeoJSON).mockResolvedValue({
			status: 200,
			data: { trees: [] } as unknown as IMarkers
		});
		logic = new TreeLayerLogic();
	});

	afterEach(() => {
		vi.unstubAllGlobals();
	});

	it('initializes with undefined bounds, zoom, markers', () => {
		expect(logic.bounds).toBeUndefined();
		expect(logic.zoom).toBeUndefined();
		expect(logic.markers).toBeUndefined();
		expect(logic.crownRadiusSmall).toBe(4);
	});

	it('handles bounds event and calls reload with getGeoJSON', async () => {
		const mockCollection = {
			type: 'FeatureCollection' as const,
			features: [
				{
					type: 'Feature' as const,
					id: 'tree-1',
					geometry: {
						type: 'Point' as const,
						coordinates: [44.5, 40.1] as [number, number]
					},
					properties: {
						id: 'tree-1',
						state: 'alive',
						type: 'Oak',
						crown: 5,
						trunk: 3
					}
				}
			]
		};

		vi.mocked(getGeoJSON).mockResolvedValueOnce({
			status: 200,
			data: mockCollection as unknown as IMarkers
		});

		const cleanup = logic.onMount();

		const bounds = {
			_sw: { lng: 44, lat: 40 },
			_ne: { lng: 45, lat: 41 },
			zoom: 16
		} as unknown as IBounds;
		mapBus.emit('bounds', bounds);

		await new Promise((resolve) => setTimeout(resolve, 150));

		expect(logic.bounds).toEqual(bounds);
		expect(logic.zoom).toBe(16);
		expect(logic.markers).toEqual(mockCollection);
		expect(mapPoiStore.trees).toEqual([
			{
				lat: 40.1,
				lon: 44.5,
				url: '/tree/tree-1'
			}
		]);

		if (cleanup) cleanup();
	});

	it('handles reload event via mapBus', async () => {
		const mockCollection = {
			type: 'FeatureCollection' as const,
			features: []
		};

		vi.mocked(getGeoJSON).mockResolvedValue({
			status: 200,
			data: mockCollection as unknown as IMarkers
		});

		const cleanup = logic.onMount();
		const bounds = {
			_sw: { lng: 44, lat: 40 },
			_ne: { lng: 45, lat: 41 },
			zoom: 16
		} as unknown as IBounds;
		mapBus.emit('bounds', bounds);
		await new Promise((resolve) => setTimeout(resolve, 150));

		const apiSpy = vi.mocked(getGeoJSON);
		apiSpy.mockClear();

		mapBus.emit('reload');
		await new Promise((resolve) => setTimeout(resolve, 150));

		expect(apiSpy).toHaveBeenCalled();

		if (cleanup) cleanup();
	});

	it('handles click event on feature when zoom is >= 15', async () => {
		logic.zoom = 16;
		const moveEmit = vi.spyOn(mapBus, 'emit');
		const vibrateSpy = vi.fn();
		vi.stubGlobal('navigator', { vibrate: vibrateSpy });

		const event = {
			features: [
				{
					properties: { id: 'tree-1' },
					geometry: { coordinates: [44.5, 40.1] }
				}
			]
		};

		await logic.handleClick(event);

		expect(moveEmit).toHaveBeenCalledWith('move', { lat: 40.1, lng: 44.5 });
		expect(routesModule.goto).toHaveBeenCalledWith('/tree/tree-1');
		expect(vibrateSpy).toHaveBeenCalledWith(50);
	});

	it('ignores click event when zoom is < 15', async () => {
		logic.zoom = 14;
		const moveEmit = vi.spyOn(mapBus, 'emit');

		const event = {
			features: [
				{
					properties: { id: 'tree-1' },
					geometry: { coordinates: [44.5, 40.1] }
				}
			]
		};

		await logic.handleClick(event);

		expect(moveEmit).not.toHaveBeenCalled();
		expect(routesModule.goto).not.toHaveBeenCalled();
	});

	it('handles context menu event on feature when zoom is >= 15', async () => {
		logic.zoom = 15;
		const menuEmit = vi.spyOn(menuBus, 'emit');
		const vibrateSpy = vi.fn();
		vi.stubGlobal('navigator', { vibrate: vibrateSpy });

		const event = {
			features: [
				{
					properties: { id: 'tree-1' },
					geometry: { coordinates: [44.5, 40.1] }
				}
			]
		};

		await logic.handleContextMenu(event);

		expect(menuEmit).toHaveBeenCalledWith('show', 'tree-1');
		expect(vibrateSpy).toHaveBeenCalledWith(50);
	});

	it('cleans up on unmount (resets bounds, zoom, markers, and clears mapPoiStore.trees)', async () => {
		logic.bounds = { _sw: { lng: 44, lat: 40 }, _ne: { lng: 45, lat: 41 } } as unknown as IBounds;
		logic.zoom = 16;
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		logic.markers = { type: 'FeatureCollection', features: [] } as any;
		mapPoiStore.trees = [{ lat: 40.1, lon: 44.5, url: '/tree/1' }];

		const cleanup = logic.onMount();
		if (cleanup) cleanup();

		expect(logic.bounds).toBeUndefined();
		expect(logic.zoom).toBeUndefined();
		expect(logic.markers).toBeUndefined();
		expect(mapPoiStore.trees).toEqual([]);
	});
});
