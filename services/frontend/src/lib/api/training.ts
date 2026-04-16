import type { IResponse } from '$lib/types';
import { getAuthHeaders, request } from './client';

export async function addTraining(result: number): Promise<IResponse<void>> {
	return await request('POST', 'v1/training', {
		body: JSON.stringify({
			result
		}),
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});
}
