import type { IExportFile, IResponse } from '$lib/types';
import { request } from './client';

export async function getExportFiles(): Promise<IResponse<IExportFile[]>> {
	return await request('GET', 'api/export/files');
}
