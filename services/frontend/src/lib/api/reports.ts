import type { IResponse } from '$lib/types';
import { request } from './client';

export interface IReportProperties {
	id: string;
	created_at: number;
	description: string | null;
	status: string;
}

export interface IReportFeature {
	type: 'Feature';
	id: string;
	geometry: {
		type: 'Point';
		coordinates: [number, number];
	};
	properties: IReportProperties;
}

export interface IReportCollection {
	type: 'FeatureCollection';
	features: IReportFeature[];
}

export interface IReport {
	id: string;
	lat: number | null;
	lon: number | null;
	created_at: number;
	description: string | null;
}

export async function getActiveReportsGeoJSON(): Promise<IResponse<IReportCollection>> {
	return await request<IReportCollection>('GET', 'v1/reports/active');
}

export async function getReport(id: string): Promise<IResponse<IReport>> {
	return await request<IReport>('GET', `v1/reports/${id}`);
}

export async function getReportPhotos(id: string): Promise<IResponse<string[]>> {
	return await request<string[]>('GET', `v1/reports/${id}/photos`);
}
