import type {
	IHeatMap,
	IResponse,
	ISpeciesStats,
	IStateStats,
	IStats,
	IStreetStats,
	ITreeList
} from '$lib/types';
import { request } from './client';

export async function getStats(): Promise<IResponse<IStats>> {
	return await request('GET', 'v1/trees/stats');
}

export async function getSpeciesStats(): Promise<IResponse<ISpeciesStats[]>> {
	return await request('GET', 'v1/stats/species');
}

export async function getSpeciesMismatch(): Promise<IResponse<ITreeList>> {
	return await request('GET', 'v1/stats/species/mismatch');
}

export async function getTopHeight(): Promise<IResponse<ITreeList>> {
	return await request('GET', 'v1/stats/height');
}

export async function getTopDiameter(): Promise<IResponse<ITreeList>> {
	return await request('GET', 'v1/stats/diameter');
}

export async function getTopCircumference(): Promise<IResponse<ITreeList>> {
	return await request('GET', 'v1/stats/circumference');
}

export async function getTopStreets(): Promise<IResponse<IStreetStats[]>> {
	return await request('GET', 'v1/stats/streets');
}

export async function getStateStats(): Promise<IResponse<IStateStats[]>> {
	return await request('GET', 'v1/stats/state');
}

export async function getHeatMap(): Promise<IResponse<IHeatMap[]>> {
	return await request('GET', 'v1/heatmap');
}
