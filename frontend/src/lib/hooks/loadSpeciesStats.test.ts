import { getSpeciesStats } from '$lib/api/stats';
import type { IResponse, ISpeciesStats } from '$lib/types';
import { get } from 'svelte/store';
import { describe, expect, it, vi } from 'vitest';
import { loadSpeciesStats } from './loadSpeciesStats';

vi.mock('$lib/api/stats', () => ({
	getSpeciesStats: vi.fn()
}));

describe('hooks/loadSpeciesStats', async () => {
	it('should load empty list', async () => {
		vi.mocked(getSpeciesStats).mockImplementation(async (): Promise<IResponse<ISpeciesStats[]>> => {
			return {
				status: 200,
				data: []
			};
		});

		const { loading, error, data, reload } = loadSpeciesStats();
		expect(get(loading)).toBe(true);

		await reload();

		expect(get(loading)).toBe(false);
		expect(get(error)).toBeUndefined();
		expect(get(data)).toEqual([]);
	});

	it('should load non-empty list', async () => {
		vi.mocked(getSpeciesStats).mockImplementation(async (): Promise<IResponse<ISpeciesStats[]>> => {
			return {
				status: 200,
				data: [
					{
						name: 'Populus',
						count: 10,
						subspecies: []
					}
				]
			};
		});

		const { loading, error, data, reload } = loadSpeciesStats();
		expect(get(loading)).toBe(true);

		await reload();

		expect(get(loading)).toBe(false);
		expect(get(error)).toBeUndefined();
		expect(get(data).length).toEqual(1);
		expect(get(data)[0].name).toEqual('Populus');
	});

	it('should return an error', async () => {
		vi.mocked(getSpeciesStats).mockImplementation(async (): Promise<IResponse<ISpeciesStats[]>> => {
			return {
				status: 500,
				data: undefined,
				error: {
					code: 'SomethingWentWrong',
					description: 'something went wrong'
				}
			};
		});

		const { loading, error, data, reload } = loadSpeciesStats();
		expect(get(loading)).toBe(true);

		await reload();

		expect(get(loading)).toBe(false);
		expect(get(data)).toEqual([]);
		expect(get(error)?.description).toEqual('something went wrong');
	});
});
