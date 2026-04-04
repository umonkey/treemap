import { config } from '$lib/env';
import type { IResponse, IStreet, StreetReport } from '$lib/types';
import { request } from './client';

export async function searchStreets(query: string): Promise<IResponse<IStreet[]>> {
	const params = new URLSearchParams({ query });
	return await request('GET', `v1/streets/search?${params}`);
}

export async function getStreetReport(address: string): Promise<IResponse<StreetReport>> {
	const params = new URLSearchParams({ address });
	return await request('GET', `v1/streets/report?${params.toString()}`);
}

export function getStreetReportCSV(address: string): string {
	const params = new URLSearchParams({ address });
	return `${config.apiRoot}v1/streets/report.csv?${params.toString()}`;
}
