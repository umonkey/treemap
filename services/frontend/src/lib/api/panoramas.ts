import type { IResponse } from '$lib/types';
import { getAuthHeaders, request } from './client';

export interface Panorama {
	id: string;
	created_at: number;
	created_by: string;
	image_count: number;
	status: string;
	title: string;
	visible: boolean;
	has_video: boolean;
	has_track: boolean;
	has_web_video: boolean;
	video_timestamp?: number;
}

export interface CreatePanorama {
	title: string;
}

export interface UpdatePanorama {
	title?: string;
	visible?: boolean;
}

export async function getPanoramas(): Promise<IResponse<Panorama[]>> {
	return await request<Panorama[]>('GET', 'api/panoramas', {
		headers: getAuthHeaders()
	});
}

export async function getPanorama(id: string): Promise<IResponse<Panorama>> {
	return await request<Panorama>('GET', `api/panoramas/${id}`, {
		headers: getAuthHeaders()
	});
}

export async function createPanorama(data: CreatePanorama): Promise<IResponse<Panorama>> {
	return await request<Panorama>('POST', 'api/panoramas', {
		headers: {
			...getAuthHeaders(),
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(data)
	});
}

export async function updatePanorama(
	id: string,
	data: UpdatePanorama
): Promise<IResponse<Panorama>> {
	return await request<Panorama>('PATCH', `api/panoramas/${id}`, {
		headers: {
			...getAuthHeaders(),
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(data)
	});
}
