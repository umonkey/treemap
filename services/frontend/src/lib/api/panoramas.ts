import type { IResponse } from '$lib/types';
import { getAuthHeaders, request } from './client';

export type PanoramaStatus =
	| 'NEEDS_FILES'
	| 'NEEDS_TRANSCODING'
	| 'NEEDS_TRANSCODING_FINISH'
	| 'NEEDS_SYNC'
	| 'NEEDS_PROCESSING'
	| 'NEEDS_PROCESSING_FINISH'
	| 'SUCCESS'
	| 'FAILURE';

export interface Panorama {
	id: string;
	created_at: number;
	created_by: string;
	image_count: number;
	status: PanoramaStatus;
	title: string;
	visible: boolean;
	source_video_path?: string | null;
	gpx_path?: string | null;
	web_video_path?: string | null;
	transcode_arn?: string | null;
	transcode_status?: string | null;
	video_timestamp?: number | null;
	gpx_offset?: number | null;
	lat_offset: number;
	lon_offset: number;
	processing_status?: string | null;
	failure_reason?: string | null;
	has_video?: boolean;
	has_track?: boolean;
	has_web_video?: boolean;
}

export interface CreatePanorama {
	title: string;
}

export interface UpdatePanorama {
	title?: string;
	visible?: boolean;
	gpx_offset?: number;
	lat_offset?: number;
	lon_offset?: number;
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

export async function verifyVideoUpload(id: string): Promise<IResponse<Panorama>> {
	return await request<Panorama>('POST', `api/panoramas/${id}/video`, {
		headers: getAuthHeaders()
	});
}

export interface MultipartUpload {
	upload_id: string;
	urls: string[];
}

export interface CompletedPart {
	part_number: number;
	etag: string;
}

export async function startVideoMultipart(
	id: string,
	partsCount: number
): Promise<IResponse<MultipartUpload>> {
	return await request<MultipartUpload>('POST', `api/panoramas/${id}/video/multipart`, {
		headers: {
			...getAuthHeaders(),
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({ parts_count: partsCount })
	});
}

export async function completeVideoMultipart(
	id: string,
	uploadId: string,
	parts: CompletedPart[]
): Promise<IResponse<Panorama>> {
	return await request<Panorama>('POST', `api/panoramas/${id}/video/multipart/complete`, {
		headers: {
			...getAuthHeaders(),
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({ upload_id: uploadId, parts })
	});
}

export interface UploadUrlResponse {
	url: string;
}

export async function getPanoramaTrackUploadUrl(id: string): Promise<IResponse<UploadUrlResponse>> {
	return await request<UploadUrlResponse>('GET', `api/panoramas/${id}/track`, {
		headers: getAuthHeaders()
	});
}

export async function uploadPanoramaTrackFile(url: string, file: File): Promise<Response> {
	return await fetch(url, {
		method: 'PUT',
		body: file
	});
}

export async function finishPanoramaTrackUpload(id: string): Promise<IResponse<Panorama>> {
	return await request<Panorama>('POST', `api/panoramas/${id}/track`, {
		headers: getAuthHeaders()
	});
}

export interface WebVideoUrlResponse {
	url: string;
}

export async function getPanoramaWebVideo(id: string): Promise<IResponse<WebVideoUrlResponse>> {
	return await request<WebVideoUrlResponse>('GET', `api/panoramas/${id}/web-video`, {
		headers: getAuthHeaders()
	});
}

export interface ITrackPoint {
	lat: number;
	lng: number;
	offset: number;
	timestamp: string;
}

export async function getPanoramaTrackData(id: string): Promise<IResponse<ITrackPoint[]>> {
	return await request<ITrackPoint[]>('GET', `api/panoramas/${id}/track.json`, {
		headers: getAuthHeaders()
	});
}

export async function getPanoramasGeoJSON(
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

	return await request<unknown>('GET', `api/panoramas/geo.json?${params.toString()}`);
}

export interface PanoramaImage {
	id: string;
	sequence_id: string;
	captured_at: number;
	lat: number;
	lon: number;
	compass_angle: number;
	url?: string;
}

export async function getPanoramasImage(id: string): Promise<IResponse<PanoramaImage>> {
	return await request<PanoramaImage>('GET', `api/panoramas/images/${id}`);
}

export async function getPanoramasHints(
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

	return await request<unknown>('GET', `api/panoramas/hints.json?${params.toString()}`);
}

export interface PanoramaHint {
	image_id: string;
	angle: number;
	user_id: string;
}

export async function getPanoramasImageHints(id: string): Promise<IResponse<PanoramaHint[]>> {
	return await request<PanoramaHint[]>('GET', `api/panoramas/images/${id}/hints`, {
		headers: getAuthHeaders()
	});
}

export async function addPanoramaImageHint(id: string, angle: number): Promise<IResponse<void>> {
	return await request<void>('POST', `api/panoramas/images/${id}/hints`, {
		headers: {
			...getAuthHeaders(),
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({ angle })
	});
}

export async function deletePanoramaImageHints(id: string): Promise<IResponse<void>> {
	return await request<void>('DELETE', `api/panoramas/images/${id}/hints`, {
		headers: getAuthHeaders()
	});
}

export async function restartPanorama(id: string): Promise<IResponse<Panorama>> {
	return await request<Panorama>('POST', `api/panoramas/${id}/restart`, {
		headers: getAuthHeaders()
	});
}

export interface PanoramaMetaExport {
	title: string;
	storage_key: string;
	created_at: number;
	gpx_offset?: number | null;
	lat_offset: number;
	lon_offset: number;
}

export interface PanoramaImageExport {
	filename: string;
	lat: number;
	lon: number;
	heading: number;
	pitch: number;
	roll: number;
}

export interface PanoramaExport {
	meta: PanoramaMetaExport;
	images: PanoramaImageExport[];
}

export async function exportPanorama(id: string): Promise<IResponse<PanoramaExport>> {
	return await request<PanoramaExport>('GET', `api/panoramas/${id}/export`, {
		headers: getAuthHeaders()
	});
}
