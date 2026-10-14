import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { TrackLayerState } from './TrackLayer.svelte.ts';
import { getPanoramaTrack, type PanoramaTrackPoint } from '$lib/api/panoramas';
import { showError } from '$lib/errors';

vi.mock('$lib/api/panoramas', () => ({
	getPanoramaTrack: vi.fn()
}));

vi.mock('$lib/errors', () => ({
	showError: vi.fn()
}));

const makePoint = (lat: number, lng: number): PanoramaTrackPoint => ({
	lat,
	lng,
	offset: 0,
	timestamp: '2026-01-01T00:00:00Z'
});

describe('TrackLayerState', () => {
	let state: TrackLayerState;

	beforeEach(() => {
		vi.clearAllMocks();
		state = new TrackLayerState();
	});

	afterEach(() => {
		vi.resetAllMocks();
	});

	it('initializes with no track data', () => {
		expect(state.trackGeoJson).toBeUndefined();
		expect(state.loading).toBe(false);
	});

	it('converts track points into a single LineString feature', async () => {
		vi.mocked(getPanoramaTrack).mockResolvedValue({
			status: 200,
			data: [makePoint(40.1, 44.5), makePoint(40.2, 44.6)]
		});

		await state.reload('1');

		expect(state.trackGeoJson).toEqual({
			type: 'FeatureCollection',
			features: [
				{
					type: 'Feature',
					geometry: {
						type: 'LineString',
						coordinates: [
							[44.5, 40.1],
							[44.6, 40.2]
						]
					},
					properties: {}
				}
			]
		});
		expect(showError).not.toHaveBeenCalled();
	});

	it('clears the track without an error when there are fewer than two points', async () => {
		vi.mocked(getPanoramaTrack).mockResolvedValue({
			status: 200,
			data: [makePoint(40.1, 44.5)]
		});

		await state.reload('1');

		expect(state.trackGeoJson).toBeUndefined();
		expect(showError).not.toHaveBeenCalled();
	});

	it('silently clears the track on a 404 response', async () => {
		vi.mocked(getPanoramaTrack).mockResolvedValue({
			status: 404,
			data: undefined,
			error: { code: 'file_not_found', description: 'File not found' }
		});

		await state.reload('1');

		expect(state.trackGeoJson).toBeUndefined();
		expect(showError).not.toHaveBeenCalled();
	});

	it('shows an error on an unexpected failure response', async () => {
		vi.mocked(getPanoramaTrack).mockResolvedValue({
			status: 500,
			data: undefined,
			error: { code: 'internal_error', description: 'Boom' }
		});

		await state.reload('1');

		expect(state.trackGeoJson).toBeUndefined();
		expect(showError).toHaveBeenCalledWith('Boom');
	});

	it('ignores stale responses from a previous panorama', async () => {
		let resolveFirst: (value: Awaited<ReturnType<typeof getPanoramaTrack>>) => void = () => {};
		vi.mocked(getPanoramaTrack).mockImplementationOnce(
			() =>
				new Promise((resolve) => {
					resolveFirst = resolve;
				})
		);
		vi.mocked(getPanoramaTrack).mockResolvedValueOnce({
			status: 200,
			data: [makePoint(40.3, 44.7), makePoint(40.4, 44.8)]
		});

		const firstReload = state.reload('1');
		await state.reload('2');
		resolveFirst({
			status: 200,
			data: [makePoint(40.1, 44.5), makePoint(40.2, 44.6)]
		});
		await firstReload;

		expect(state.trackGeoJson?.features[0].geometry).toEqual({
			type: 'LineString',
			coordinates: [
				[44.7, 40.3],
				[44.8, 40.4]
			]
		});
	});
});
