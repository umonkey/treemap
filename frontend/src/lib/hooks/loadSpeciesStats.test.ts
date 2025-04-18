import { apiClient } from '$lib/api';
import type { IResponse, ISpeciesStats } from '$lib/types';
import { get } from 'svelte/store';
import { describe, expect, it } from 'vitest';
import { loadSpeciesStats } from './loadSpeciesStats';

describe('hooks/loadSpeciesStats', async () => {
	it('should load empty list', async () => {
		apiClient.getSpeciesStats = async (): Promise<IResponse<ISpeciesStats[]>> => {
			return {
				status: 200,
				data: []
			};
		};

		const { loading, error, data, reload } = loadSpeciesStats();
		expect(get(loading)).toBe(true);

		await reload();

		expect(get(loading)).toBe(false);
		expect(get(error)).toBeUndefined();
		expect(get(data)).toEqual([]);
	});

	it('should load non-empty list', async () => {
		apiClient.getSpeciesStats = async (): Promise<IResponse<ISpeciesStats[]>> => {
			return {
				status: 200,
				data: [
					{
						species: 'Populus',
						count: 10
					}
				]
			};
		};

		const { loading, error, data, reload } = loadSpeciesStats();
		expect(get(loading)).toBe(true);

		await reload();

		expect(get(loading)).toBe(false);
		expect(get(error)).toBeUndefined();
		expect(get(data).length).toEqual(1);
		expect(get(data)[0].species).toEqual('Populus');
	});

	it('should return an error', async () => {
		apiClient.getSpeciesStats = async (): Promise<IResponse<ISpeciesStats[]>> => {
			return {
				status: 500,
				data: undefined,
				error: {
					code: 'SomethingWentWrong',
					description: 'something went wrong'
				}
			};
		};

		const { loading, error, data, reload } = loadSpeciesStats();
		expect(get(loading)).toBe(true);

		await reload();

		expect(get(loading)).toBe(false);
		expect(get(data)).toEqual([]);
		expect(get(error)?.description).toEqual('something went wrong');
	});
});
