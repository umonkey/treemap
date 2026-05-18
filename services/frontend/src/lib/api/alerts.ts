import type { IResponse } from '$lib/types';
import { request } from './client';

export interface IAlertProperties {
	id: string;
	created_at: number;
	description: string | null;
	status: string;
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
}

export async function getActiveAlertsGeoJSON(): Promise<IResponse<IAlertCollection>> {
	return await request<IAlertCollection>('GET', 'v1/alerts/active');
}

export async function getAlert(id: string): Promise<IResponse<IAlert>> {
	return await request<IAlert>('GET', `v1/alerts/${id}`);
}

export async function getAlertPhotos(id: string): Promise<IResponse<string[]>> {
	return await request<string[]>('GET', `v1/alerts/${id}/photos`);
}
