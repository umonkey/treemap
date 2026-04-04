import type { IObservation, IResponse } from '$lib/types';
import { getAuthHeaders, request } from './client';

export async function getObservations(treeId: string): Promise<IResponse<IObservation>> {
	return await request('GET', `v1/trees/${treeId}/observations`);
}

export async function addObservations(observation: IObservation): Promise<IResponse<IObservation>> {
	return await request('POST', `v1/trees/${observation.tree_id}/observations`, {
		body: JSON.stringify(observation),
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});
}
