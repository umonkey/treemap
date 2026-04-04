import type { IResponse, ISpecies } from '$lib/types';
import { getAuthHeaders, request } from './client';

export async function searchSpecies(query: string): Promise<IResponse<ISpecies[]>> {
	const params = new URLSearchParams({ query });
	return await request('GET', `v1/species/search?${params}`);
}

export async function suggestSpecies(): Promise<IResponse<string[]>> {
	return await request('GET', 'v1/species/suggest', {
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});
}
