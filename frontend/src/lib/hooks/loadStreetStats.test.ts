import { getTopStreets } from '$lib/api/stats';
import type { IResponse, IStreetStats } from '$lib/types';
import { get } from 'svelte/store';
import { describe, expect, it, vi } from 'vitest';
import { loadStreetStats } from './loadStreetStats';

vi.mock('$lib/api/stats', () => ({
	getTopStreets: vi.fn()
}));

describe('hooks/loadStreetStats', async () => {
	it('should load empty list', async () => {
		vi.mocked(getTopStreets).mockImplementation(async (): Promise<IResponse<IStreetStats[]>> => {
			return {
				status: 200,
				data: []
			};
		});

		const { loading, error, data, reload } = loadStreetStats();
		expect(get(loading)).toBe(true);

		await reload();

		expect(get(loading)).toBe(false);
		expect(get(error)).toBeUndefined();
		expect(get(data)).toEqual([]);
	});

	it('should load non-empty list', async () => {
		vi.mocked(getTopStreets).mockImplementation(async (): Promise<IResponse<IStreetStats[]>> => {
			return {
				status: 200,
				data: [
					{
						address: 'Tumanyan str.',
						count: 10
					}
				]
			};
		});

		const { loading, error, data, reload } = loadStreetStats();
		expect(get(loading)).toBe(true);

		await reload();

		expect(get(loading)).toBe(false);
		expect(get(error)).toBeUndefined();
		expect(get(data).length).toEqual(1);
		expect(get(data)[0].address).toEqual('Tumanyan str.');
	});

	it('should return an error', async () => {
		vi.mocked(getTopStreets).mockImplementation(async (): Promise<IResponse<IStreetStats[]>> => {
			return {
				status: 500,
				data: undefined,
				error: {
					code: 'SomethingWentWrong',
					description: 'something went wrong'
				}
			};
		});

		const { loading, error, data, reload } = loadStreetStats();
		expect(get(loading)).toBe(true);

		await reload();

		expect(get(loading)).toBe(false);
		expect(get(data)).toEqual([]);
		expect(get(error)?.description).toEqual('something went wrong');
	});
});
