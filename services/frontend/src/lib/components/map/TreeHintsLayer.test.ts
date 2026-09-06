import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { TreeHintsLayerLogic } from './TreeHintsLayer.svelte.ts';
import { mapBus } from '$lib/buses/mapBus';
import { mapState } from './MapLibre.svelte.ts';
import { getPanoramasHints } from '$lib/api/panoramas';
import type { IBounds } from '$lib/types';

vi.mock('$lib/api/panoramas', () => ({
	getPanoramasHints: vi.fn()
}));

describe('TreeHintsLayerLogic', () => {
	let logic: TreeHintsLayerLogic;

	beforeEach(() => {
		vi.clearAllMocks();
		mapState.treeHintsLayer = true;
		vi.mocked(getPanoramasHints).mockResolvedValue({
			status: 200,
			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			data: { type: 'FeatureCollection', features: [] } as any
		});
		logic = new TreeHintsLayerLogic();
	});

	afterEach(() => {
		// cleanup
	});

	it('initializes with undefined data and bounds', () => {
		expect(logic.data).toBeUndefined();
		expect(logic.bounds).toBeUndefined();
	});

	it('handles bounds and calls reload when treeHintsLayer is enabled', async () => {
		const mockCollection = {
			type: 'FeatureCollection' as const,
			features: [
				{
					type: 'Feature' as const,
					id: 'hint-1',
					geometry: {
						type: 'LineString' as const,
						coordinates: [
							[44.5, 40.1],
							[44.6, 40.2]
						]
					},
					properties: {}
				}
			]
		};

		vi.mocked(getPanoramasHints).mockResolvedValueOnce({
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

		if (cleanup) cleanup();
	});

	it('does not reload when treeHintsLayer is disabled', async () => {
		mapState.treeHintsLayer = false;
		const apiSpy = vi.mocked(getPanoramasHints);
		apiSpy.mockClear();

		const cleanup = logic.onMount();
		const bounds = { _sw: { lng: 44, lat: 40 }, _ne: { lng: 45, lat: 41 } } as unknown as IBounds;
		mapBus.emit('bounds', bounds);

		await new Promise((resolve) => setTimeout(resolve, 250));

		expect(apiSpy).not.toHaveBeenCalled();

		if (cleanup) cleanup();
	});
});
