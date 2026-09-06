import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { AlertLayerLogic } from './AlertLayer.svelte.ts';
import { mapLayerStore } from '$lib/stores/mapLayerStore';
import { mapBus } from '$lib/buses/mapBus';
import { mapPoiStore } from '$lib/stores/mapPoi.svelte';
import { getActiveAlertsGeoJSON } from '$lib/api/alerts';
import * as routesModule from '$lib/routes';

vi.mock('$lib/api/alerts', () => ({
	getActiveAlertsGeoJSON: vi.fn()
}));

vi.mock('$lib/routes', async () => {
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	const actual = (await vi.importActual('$lib/routes')) as any;
	return {
		...actual,
		goto: vi.fn().mockResolvedValue(undefined),
		routes: {
			...actual.routes,
			alertPreview: (id: string) => `/alert/${id}`
		}
	};
});

vi.mock('svelte-maplibre', () => ({
	getMapContext: () => ({
		map: {}
	})
}));

describe('AlertLayerLogic', () => {
	let logic: AlertLayerLogic;

	beforeEach(() => {
		vi.clearAllMocks();
		mapLayerStore.set({
			base: 'light',
			alerts: true,
			drone: false,
			panoramas: false,
			treeHints: false,
			stickyPoints: true
		});
		mapPoiStore.alerts = [];
		vi.mocked(getActiveAlertsGeoJSON).mockResolvedValue({
			status: 200,
			data: { type: 'FeatureCollection', features: [] }
		});
		logic = new AlertLayerLogic();
	});

	afterEach(() => {
		// cleanup
	});

	it('initializes enabled from mapLayerStore', () => {
		expect(logic.enabled).toBe(true);
		expect(logic.alerts).toBe(true);

		mapLayerStore.set({
			base: 'light',
			alerts: false,
			drone: false,
			panoramas: false,
			treeHints: false,
			stickyPoints: true
		});
		const logic2 = new AlertLayerLogic();
		expect(logic2.enabled).toBe(false);
		expect(logic2.alerts).toBe(false);
	});

	it('handles reload when enabled', async () => {
		const mockCollection = {
			type: 'FeatureCollection' as const,
			features: [
				{
					type: 'Feature' as const,
					id: 'alert-1',
					geometry: {
						type: 'Point' as const,
						coordinates: [44.5, 40.1] as [number, number]
					},
					properties: {
						id: 'alert-1',
						created_at: 123456789,
						description: 'Test alert',
						status: 'new',
						weight: 1
					}
				}
			]
		};

		vi.mocked(getActiveAlertsGeoJSON).mockResolvedValueOnce({
			status: 200,
			data: mockCollection
		});

		const cleanup = logic.onMount();
		await new Promise((resolve) => setTimeout(resolve, 150));

		expect(logic.markers).toEqual(mockCollection);
		expect(mapPoiStore.alerts).toEqual([
			{
				lat: 40.1,
				lon: 44.5,
				url: '/alert/alert-1'
			}
		]);

		if (cleanup) cleanup();
	});

	it('exits reload early when disabled', async () => {
		mapLayerStore.set({
			base: 'light',
			alerts: false,
			drone: false,
			panoramas: false,
			treeHints: false,
			stickyPoints: true
		});
		const logicDisabled = new AlertLayerLogic();
		const apiSpy = vi.mocked(getActiveAlertsGeoJSON);
		apiSpy.mockClear();

		const cleanup = logicDisabled.onMount();
		await new Promise((resolve) => setTimeout(resolve, 150));

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
					properties: { id: 'alert-1' },
					geometry: { coordinates: [44.5, 40.1] }
				}
			]
		};

		await logic.handleClick(event);

		expect(moveEmit).toHaveBeenCalledWith('move', { lat: 40.1, lng: 44.5 });
		expect(routesModule.goto).toHaveBeenCalledWith('/alert/alert-1');
		expect(vibrateSpy).toHaveBeenCalledWith(50);

		vi.unstubAllGlobals();
	});

	it('subscribes to mapLayerStore changes in onMount', async () => {
		mapLayerStore.set({
			base: 'light',
			alerts: false,
			drone: false,
			panoramas: false,
			treeHints: false,
			stickyPoints: true
		});
		const logicSub = new AlertLayerLogic();
		const apiSpy = vi.mocked(getActiveAlertsGeoJSON);
		apiSpy.mockClear();

		const cleanup = logicSub.onMount();

		expect(logicSub.enabled).toBe(false);

		mapLayerStore.set({
			base: 'light',
			alerts: true,
			drone: false,
			panoramas: false,
			treeHints: false,
			stickyPoints: true
		});

		expect(logicSub.enabled).toBe(true);
		await new Promise((resolve) => setTimeout(resolve, 150));
		expect(apiSpy).toHaveBeenCalled();

		if (cleanup) cleanup();
	});
});
