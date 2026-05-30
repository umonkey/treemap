import type { IResponse } from '$lib/types';
import { request } from './client';

export async function getMapillaryGeoJSON(
	n: number,
	e: number,
	s: number,
	w: number,
	points: boolean = true,
	lines: boolean = true
): Promise<IResponse<unknown>> {
	const params = new URLSearchParams({
		n: n.toString(),
		e: e.toString(),
		s: s.toString(),
		w: w.toString()
	});

	if (points) {
		params.set('points', 'true');
	}

	if (lines) {
		params.set('lines', 'true');
	}

	return await request<unknown>('GET', `v1/mapillary/geo.json?${params.toString()}`);
}

export interface MapillaryImage {
	id: string;
	sequence_id: string;
	captured_at: number;
	lat: number;
	lon: number;
	compass_angle: number;
	url?: string;
}

export async function getMapillaryImage(id: string): Promise<IResponse<MapillaryImage>> {
	return await request<MapillaryImage>('GET', `v1/mapillary/images/${id}`);
}
