import type { IResponse } from '$lib/types';
import { getAuthHeaders, request } from './client';

export interface IAlertProperties {
	id: string;
	created_at: number;
	description: string | null;
	status: string;
	weight: number;
}

export interface IAlertFeature {
	type: 'Feature';
	id: string;
	geometry: {
		type: 'Point';
		coordinates: [number, number];
	};
	properties: IAlertProperties;
}

export interface IAlertCollection {
	type: 'FeatureCollection';
	features: IAlertFeature[];
}

export interface IAlert {
	id: string;
	lat: number | null;
	lon: number | null;
	created_at: number;
	description: string | null;
	source: 'web' | 'telegram';
	editable: boolean;
}

export interface IAddAlertRequest {
	lat: number;
	lon: number;
	description: string;
	files: string[];
}

export interface IUpdateAlertRequest {
	description: string;
}

export async function getActiveAlertsGeoJSON(): Promise<IResponse<IAlertCollection>> {
	return await request<IAlertCollection>('GET', 'v1/alerts/active');
}

export async function addAlert(req: IAddAlertRequest): Promise<IResponse<IAlert>> {
	return await request<IAlert>('POST', 'v1/alerts', {
		body: JSON.stringify(req),
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});
}

export async function getAlert(id: string): Promise<IResponse<IAlert>> {
	return await request<IAlert>('GET', `v1/alerts/${id}`, {
		headers: getAuthHeaders()
	});
}

export async function updateAlert(
	id: string,
	req: IUpdateAlertRequest
): Promise<IResponse<IAlert>> {
	return await request<IAlert>('PATCH', `v1/alerts/${id}`, {
		body: JSON.stringify(req),
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});
}

export async function getAlertPhotos(id: string): Promise<IResponse<string[]>> {
	return await request<string[]>('GET', `v1/alerts/${id}/photos`);
}
