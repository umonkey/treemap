import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { mapLayerStore } from '$lib/stores/mapLayerStore';
import { mapPoiStore } from '$lib/stores/mapPoi.svelte';
import * as routesModule from '$lib/routes';
import type { Map, MapLibreEvent } from 'maplibre-gl';
import { NearestPoiLogic } from './NearestPoi.svelte.ts';

vi.mock('$lib/routes', async () => {
	const actual = await vi.importActual<typeof import('$lib/routes')>('$lib/routes');

	return {
		...actual,
		goto: vi.fn().mockResolvedValue(undefined)
	};
});

const setLayers = (center: boolean, stickyPoints: boolean) => {
	mapLayerStore.set({
		base: 'light',
		drone: false,
		alerts: true,
		panoramas: false,
		treeHints: false,
		water: true,
		genus: true,
		stickyPoints,
		center,
		missingHeight: false,
		missingDiameter: false,
		missingCircumference: false,
		missingObservations: false,
		missingPhotos: false
	});
};

const createMap = () => {
	const handlers: Record<string, (event: MapLibreEvent) => void> = {};

	const map = {
		getCenter: () => ({ lat: 0, lng: 0 }),
		getZoom: () => 20,
		on: (event: string, handler: (event: MapLibreEvent) => void) => {
			handlers[event] = handler;
		},
		off: vi.fn()
	} as unknown as Map;

	return { map, handlers };
};

describe('NearestPoiLogic', () => {
	beforeEach(() => {
		vi.clearAllMocks();
		vi.spyOn(mapPoiStore, 'getNearest').mockReturnValue({
			poi: { lat: 0, lon: 0, url: '/tree/1' },
			distance: 0.5
		});
	});

	afterEach(() => {
		vi.restoreAllMocks();
	});

	it('does not navigate to the nearest POI when the crosshair is off', () => {
		setLayers(false, true);

		const logic = new NearestPoiLogic();
		const { map, handlers } = createMap();
		logic.mount(map);

		handlers['moveend']?.({ originalEvent: {} } as unknown as MapLibreEvent);

		expect(routesModule.goto).not.toHaveBeenCalled();
	});

	it('navigates to the nearest POI when the crosshair is on', () => {
		setLayers(true, true);

		const logic = new NearestPoiLogic();
		const { map, handlers } = createMap();
		logic.mount(map);

		handlers['moveend']?.({ originalEvent: {} } as unknown as MapLibreEvent);

		expect(routesModule.goto).toHaveBeenCalledTimes(1);
		expect(routesModule.goto).toHaveBeenCalledWith('/tree/1');
	});
});
