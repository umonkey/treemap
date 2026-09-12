import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { PanoramicLayerLogic, fixSequenceFormat } from './PanoramicLayer.svelte.ts';
import { mapBus } from '$lib/buses/mapBus';
import { mapPoiStore } from '$lib/stores/mapPoi.svelte';
import { mapState } from './MapLibre.svelte.ts';
import { getPanoramasGeoJSON } from '$lib/api/panoramas';
import * as routesModule from '$lib/routes';
import type { IBounds } from '$lib/types';

vi.mock('$lib/api/panoramas', () => ({
	getPanoramasGeoJSON: vi.fn()
}));

vi.mock('$lib/routes', async () => {
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	const actual = (await vi.importActual('$lib/routes')) as any;
	return {
		...actual,
		goto: vi.fn().mockResolvedValue(undefined),
		routes: {
			...actual.routes,
			panorama: (id: string) => `/panorama/${id}`
		}
	};
});

describe('fixSequenceFormat', () => {
	it('wraps depth-2 coordinates into a single section', () => {
		const flat = [
			[44.5, 40.1],
			[44.6, 40.2]
		];

		expect(fixSequenceFormat(flat)).toEqual([
			[
				[44.5, 40.1],
				[44.6, 40.2]
			]
		]);
	});

	it('passes depth-3 coordinates through unchanged', () => {
		const sections = [
			[
				[44.5, 40.1],
				[44.6, 40.2]
			]
		];

		expect(fixSequenceFormat(sections)).toEqual(sections);
	});

	it('returns an empty array for empty coordinates', () => {
		expect(fixSequenceFormat([])).toEqual([]);
	});
});

describe('PanoramicLayerLogic', () => {
	let logic: PanoramicLayerLogic;

	beforeEach(() => {
		vi.clearAllMocks();
		mapState.panoramasLayer = true;
		mapPoiStore.panoramas = [];
		vi.mocked(getPanoramasGeoJSON).mockResolvedValue({
			status: 200,
			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			data: { type: 'FeatureCollection', features: [] } as any
		});
		logic = new PanoramicLayerLogic();
	});

	afterEach(() => {
		// cleanup
	});

	it('initializes with undefined data and bounds', () => {
		expect(logic.data).toBeUndefined();
		expect(logic.bounds).toBeUndefined();
	});

	it('handles bounds and calls reload when panoramasLayer is enabled', async () => {
		const mockCollection = {
			type: 'FeatureCollection' as const,
			features: [
				{
					type: 'Feature' as const,
					id: 'pano-1',
					geometry: {
						type: 'Point' as const,
						coordinates: [44.5, 40.1] as [number, number]
					},
					properties: {
						id: 'pano-1',
						kind: 'image' as const,
						captured_at: 123456789
					}
				}
			]
		};

		vi.mocked(getPanoramasGeoJSON).mockResolvedValueOnce({
			status: 200,
			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			data: mockCollection as any
		});

		const cleanup = logic.onMount();

		const bounds = { _sw: { lng: 44, lat: 40 }, _ne: { lng: 45, lat: 41 } } as unknown as IBounds;
		mapBus.emit('bounds', bounds);

		await new Promise((resolve) => setTimeout(resolve, 250));

		expect(logic.bounds).toEqual(bounds);
		expect(logic.data).toEqual(mockCollection);
		expect(mapPoiStore.panoramas).toEqual([
			{
				lat: 40.1,
				lon: 44.5,
				url: '/panorama/pano-1'
			}
		]);

		if (cleanup) cleanup();
	});

	it('normalizes a legacy LineString sequence feature to a MultiLineString', async () => {
		const mockCollection = {
			type: 'FeatureCollection' as const,
			features: [
				{
					type: 'Feature' as const,
					id: 'pano-seq-1',
					geometry: {
						type: 'LineString' as const,
						coordinates: [
							[44.5, 40.1],
							[44.6, 40.2]
						]
					},
					properties: {
						id: 'pano-seq-1',
						kind: 'sequence' as const,
						captured_at: 123456789
					}
				}
			]
		};

		vi.mocked(getPanoramasGeoJSON).mockResolvedValueOnce({
			status: 200,
			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			data: mockCollection as any
		});

		const cleanup = logic.onMount();
		const bounds = { _sw: { lng: 44, lat: 40 }, _ne: { lng: 45, lat: 41 } } as unknown as IBounds;
		mapBus.emit('bounds', bounds);

		await new Promise((resolve) => setTimeout(resolve, 250));

		const feature = logic.data?.features[0];

		expect(feature?.geometry.type).toBe('MultiLineString');
		expect(feature?.geometry.coordinates).toEqual([
			[
				[44.5, 40.1],
				[44.6, 40.2]
			]
		]);
		expect(mapPoiStore.panoramas).toEqual([]);

		if (cleanup) cleanup();
	});

	it('does not reload when panoramasLayer is disabled', async () => {
		mapState.panoramasLayer = false;
		const apiSpy = vi.mocked(getPanoramasGeoJSON);
		apiSpy.mockClear();

		const cleanup = logic.onMount();
		const bounds = { _sw: { lng: 44, lat: 40 }, _ne: { lng: 45, lat: 41 } } as unknown as IBounds;
		mapBus.emit('bounds', bounds);

		await new Promise((resolve) => setTimeout(resolve, 250));

		expect(apiSpy).not.toHaveBeenCalled();

		if (cleanup) cleanup();
	});

	it('handles click on feature', async () => {
		const moveEmit = vi.spyOn(mapBus, 'emit');
		const vibrateSpy = vi.fn();
		vi.stubGlobal('navigator', { vibrate: vibrateSpy });

		const event = {
			features: [
				{
					properties: { id: 'pano-1' },
					geometry: { coordinates: [44.5, 40.1] }
				}
			]
		};

		await logic.handleClick(event);

		expect(moveEmit).toHaveBeenCalledWith('pin', { lat: 40.1, lng: 44.5 });
		expect(moveEmit).toHaveBeenCalledWith('move', { lat: 40.1, lng: 44.5 });
		expect(routesModule.goto).toHaveBeenCalledWith('/panorama/pano-1');
		expect(vibrateSpy).toHaveBeenCalledWith(50);

		vi.unstubAllGlobals();
	});
});
