import type { IResponse } from '$lib/types';
import { getAuthHeaders, request } from './client';

export interface IInstance {
	name: string;
	description: string | null;
}

export async function getInstance(): Promise<IResponse<IInstance>> {
	return await request<IInstance>('GET', 'api/instance', {
		headers: { 'Content-Type': 'application/json', ...getAuthHeaders() }
	});
}

export async function updateInstance(props: {
	name?: string;
	description?: string | null;
}): Promise<IResponse<void>> {
	return await request<void>('PATCH', 'api/instance', {
		body: JSON.stringify(props),
		headers: { 'Content-Type': 'application/json', ...getAuthHeaders() }
	});
}
