import type { IResponse, IWaterSource } from '$lib/types';
import { getAuthHeaders, request } from './client';

export type IWaterFeature = {
	type: 'Feature';
	id: string;
	geometry: {
		type: 'Point';
		coordinates: [number, number];
	};
	properties: {
		id: string;
		status: string;
		created_at: number;
	};
};

export type IWaterCollection = {
	type: 'FeatureCollection';
	features: IWaterFeature[];
};

// Return water sources within the given bounds as GeoJSON.
export async function getWaterGeoJSON(
	n: number,
	e: number,
	s: number,
	w: number
): Promise<IResponse<IWaterCollection>> {
	const params = new URLSearchParams({
		n: n.toString(),
		e: e.toString(),
		s: s.toString(),
		w: w.toString()
	});

	return await request<IWaterCollection>('GET', `v1/water/geo.json?${params.toString()}`);
}

// Return a single water source.
export async function getWater(id: string): Promise<IResponse<IWaterSource>> {
	return await request<IWaterSource>('GET', `v1/water/${id}`);
}

// Add a new water source.
export async function addWater(lat: number, lon: number): Promise<IResponse<IWaterSource>> {
	return await request('POST', 'v1/water', {
		body: JSON.stringify({ lat, lon }),
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});
}

// Update the coordinates of a water source.
export async function updateWaterLocation(
	id: string,
	lat: number,
	lon: number
): Promise<IResponse<IWaterSource>> {
	return await request('PATCH', `v1/water/${id}`, {
		body: JSON.stringify({ lat, lon }),
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});
}
