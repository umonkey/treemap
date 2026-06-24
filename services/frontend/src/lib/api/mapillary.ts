import type { IResponse } from '$lib/types';
import { getAuthHeaders, request } from './client';

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

export async function getMapillaryHints(
	n: number,
	e: number,
	s: number,
	w: number
): Promise<IResponse<unknown>> {
	const params = new URLSearchParams({
		n: n.toString(),
		e: e.toString(),
		s: s.toString(),
		w: w.toString()
	});

	return await request<unknown>('GET', `v1/mapillary/hints.json?${params.toString()}`);
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

export interface MapillaryTree {
	image_id: string;
	angle: number;
	tree_id?: number;
	user_id: number;
}

export async function getMapillaryImageTrees(id: string): Promise<IResponse<MapillaryTree[]>> {
	return await request<MapillaryTree[]>('GET', `v1/mapillary/images/${id}/trees`, {
		headers: getAuthHeaders()
	});
}

export async function addMapillaryTree(id: string, angle: number): Promise<IResponse<void>> {
	return await request<void>('POST', `v1/mapillary/images/${id}/trees`, {
		headers: {
			...getAuthHeaders(),
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({ angle })
	});
}

export async function deleteMapillaryTrees(id: string): Promise<IResponse<void>> {
	return await request<void>('DELETE', `v1/mapillary/images/${id}/trees`, {
		headers: getAuthHeaders()
	});
}

export interface MapillarySequenceSummary {
	id: string;
	captured_at: number;
	image_count: number;
	hidden: boolean;
	title: string;
}

export interface MapillarySequenceDetail {
	id: string;
	captured_at: number;
	image_count: number;
	min_lat: number;
	max_lat: number;
	min_lon: number;
	max_lon: number;
	hidden: boolean;
	title: string;
	lat_offset: number;
	lon_offset: number;
}

export async function getMapillarySequences(): Promise<IResponse<MapillarySequenceSummary[]>> {
	return await request<MapillarySequenceSummary[]>('GET', 'v1/mapillary/sequences', {
		headers: getAuthHeaders()
	});
}

export async function getMapillarySequence(
	id: string
): Promise<IResponse<MapillarySequenceDetail>> {
	return await request<MapillarySequenceDetail>('GET', `v1/mapillary/sequences/${id}`, {
		headers: getAuthHeaders()
	});
}

export async function updateMapillarySequence(
	id: string,
	data: { title?: string; hidden?: boolean; lat_offset?: number; lon_offset?: number }
): Promise<IResponse<void>> {
	return await request<void>('PATCH', `v1/mapillary/sequences/${id}`, {
		headers: {
			...getAuthHeaders(),
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(data)
	});
}
