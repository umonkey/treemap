import { getPanoramasHints } from '$lib/api/panoramas';
import { mapBus } from '$lib/buses/mapBus';
import { panoBus } from '$lib/buses/panoBus';
import { showError } from '$lib/errors';
import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest';
import { TreeHintsLayerState } from './TreeHintsLayer.svelte.ts';

const { mapEventBus, panoEventBus } = vi.hoisted(() => {
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
		panoEventBus: createBus()
	};
});

vi.mock('$lib/api/panoramas', () => ({
	getPanoramasHints: vi.fn()
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

vi.mock('$lib/buses/panoBus', () => ({
	panoBus: {
		emit: vi.fn((type, evt) => panoEventBus.emit(type, evt)),
		on: vi.fn((type, handler) => panoEventBus.on(type, handler)),
		off: vi.fn((type, handler) => panoEventBus.off(type, handler))
	}
}));

const mockedGetPanoramasHints = vi.mocked(getPanoramasHints);
const mockedShowError = vi.mocked(showError);
const mockedMapBusOn = vi.mocked(mapBus.on);
const mockedMapBusOff = vi.mocked(mapBus.off);
const mockedPanoBusOn = vi.mocked(panoBus.on);
const mockedPanoBusOff = vi.mocked(panoBus.off);

const mockCollection = {
	type: 'FeatureCollection' as const,
	features: [
		{
			type: 'Feature' as const,
			id: 'hint-1',
			geometry: {
				type: 'LineString',
				coordinates: [
					[44.51, 40.18],
					[44.52, 40.19]
				]
			},
			properties: {
				id: 'hint-1'
			}
		}
	]
};

describe('TreeHintsLayerState', () => {
	beforeEach(() => {
		vi.useFakeTimers();
		mockedGetPanoramasHints.mockReset();
		mockedShowError.mockClear();
		mockedMapBusOn.mockClear();
		mockedMapBusOff.mockClear();
		mockedPanoBusOn.mockClear();
		mockedPanoBusOff.mockClear();
		mapEventBus.clear();
		panoEventBus.clear();
	});

	afterEach(() => {
		vi.useRealTimers();
	});

	test('initial state has no data and undefined bounds', () => {
		const state = new TreeHintsLayerState();

		expect(state.data).toBeUndefined();
		expect(state.bounds).toBeUndefined();
	});

	test('onMount registers listeners and cleanup unregisters them', () => {
		const state = new TreeHintsLayerState();
		const cleanup = state.onMount();

		expect(mockedMapBusOn).toHaveBeenCalledWith('bounds', expect.any(Function));
		expect(mockedPanoBusOn).toHaveBeenCalledWith('reload', expect.any(Function));

		state.bounds = { n: 40.2, s: 40.0, e: 44.6, w: 44.4 };

		cleanup();

		expect(state.bounds).toBeUndefined();
		expect(mockedMapBusOff).toHaveBeenCalledWith('bounds', expect.any(Function));
		expect(mockedPanoBusOff).toHaveBeenCalledWith('reload', expect.any(Function));
	});

	test('receiving mapBus bounds updates bounds and triggers getPanoramasHints with extended bounds', async () => {
		const state = new TreeHintsLayerState();
		state.onMount();

		mockedGetPanoramasHints.mockResolvedValueOnce({
			status: 200,
			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			data: mockCollection as any
		});

		const inputBounds = { n: 40.5, s: 39.5, e: 44.5, w: 43.5 };
		mapBus.emit('bounds', inputBounds);

		expect(state.bounds).toEqual(inputBounds);

		await vi.advanceTimersByTimeAsync(250);

		// dLat = 1.0, dLon = 1.0 -> n = 41.5, e = 45.5, s = 38.5, w = 42.5
		expect(mockedGetPanoramasHints).toHaveBeenCalledWith(41.5, 45.5, 38.5, 42.5);
		expect(state.data).toEqual(mockCollection);
	});

	test('receiving panoBus reload reloads data when bounds are present and skips when bounds are not set', async () => {
		const state = new TreeHintsLayerState();
		state.onMount();

		// When bounds are not set, reload does nothing
		panoBus.emit('reload');
		await vi.advanceTimersByTimeAsync(250);
		expect(mockedGetPanoramasHints).not.toHaveBeenCalled();

		// Now set bounds
		mockedGetPanoramasHints.mockResolvedValue({
			status: 200,
			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			data: mockCollection as any
		});

		mapBus.emit('bounds', { n: 40.5, s: 39.5, e: 44.5, w: 43.5 });
		await vi.advanceTimersByTimeAsync(250);
		expect(mockedGetPanoramasHints).toHaveBeenCalledTimes(1);

		mockedGetPanoramasHints.mockClear();

		// Trigger reload
		panoBus.emit('reload');
		await vi.advanceTimersByTimeAsync(250);
		expect(mockedGetPanoramasHints).toHaveBeenCalledTimes(1);
		expect(mockedGetPanoramasHints).toHaveBeenCalledWith(41.5, 45.5, 38.5, 42.5);
	});

	test('handles getPanoramasHints error and shows error toast', async () => {
		const state = new TreeHintsLayerState();
		state.onMount();

		mockedGetPanoramasHints.mockRejectedValueOnce(new Error('Network error'));

		mapBus.emit('bounds', { n: 40.5, s: 39.5, e: 44.5, w: 43.5 });
		await vi.advanceTimersByTimeAsync(250);

		expect(mockedShowError).toHaveBeenCalledWith('Error loading tree hints, please try again.');
	});
});
