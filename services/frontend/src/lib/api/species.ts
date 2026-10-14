import type { IResponse, ISpecies, ISpeciesDetails, ISpeciesListItem } from '$lib/types';
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

// Return all known species.
export async function listSpecies(): Promise<IResponse<ISpeciesListItem[]>> {
	return await request<ISpeciesListItem[]>('GET', 'v1/species/list');
}

// Return a single species by its latin name.
export async function getSpecies(name: string): Promise<IResponse<ISpeciesDetails>> {
	return await request<ISpeciesDetails>('GET', `v1/species/${encodeURIComponent(name)}`);
}

// Create a new species record.
export async function addSpecies(payload: {
	name: string;
	local: string;
	keywords: string[];
	wikidata_id: string;
}): Promise<IResponse<ISpeciesDetails>> {
	return await request<ISpeciesDetails>('POST', 'v1/species', {
		body: JSON.stringify(payload),
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});
}

// Update an existing species record.
export async function updateSpecies(
	name: string,
	payload: {
		local: string;
		keywords: string[];
		wikidata_id: string;
	}
): Promise<IResponse<ISpeciesDetails>> {
	return await request<ISpeciesDetails>('PUT', `v1/species/${encodeURIComponent(name)}`, {
		body: JSON.stringify(payload),
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});
}
