import type { IResponse } from '$lib/types';
import { request } from './client';

export interface ISettings {
	name: string;
	description: string | null;
	bbox: [number, number, number, number] | null;
}

export async function getSettings(): Promise<IResponse<ISettings>> {
	return await request<ISettings>('GET', 'api/settings');
}
