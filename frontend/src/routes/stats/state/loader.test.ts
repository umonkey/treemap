import { describe, it, expect } from 'vitest';
import { apiClient } from '$lib/api';
import type { Response } from '$lib/api';
import { loader } from './loader';
import type { IStateStats } from '$lib/types';

describe('routes/stats/state', async () => {
	it('should load empty list', async () => {
		apiClient.getStateStats = async (): Promise<Response<IStateStats[]>> => {
			return {
				status: 200,
				data: []
			};
		};

		const { error, stats } = await loader();

		expect(error).toBeNull();
		expect(stats).toEqual([]);
	});

	it('should load non-empty list', async () => {
		apiClient.getStateStats = async (): Promise<Response<IStateStats[]>> => {
			return {
				status: 200,
				data: [
					{
						state: 'dead',
						count: 10
					}
				]
			};
		};

		const { error, stats } = await loader();

		expect(error).toBeNull();
		expect(stats.length).toEqual(1);
		expect(stats[0].state).toEqual('dead');
	});

	it('should return an error', async () => {
		apiClient.getStateStats = async (): Promise<Response<IStateStats[]>> => {
			return {
				status: 500,
				data: undefined,
				error: {
					error: {
						code: 'SomethingWentWrong',
						description: 'something went wrong'
					}
				}
			};
		};

		const { error, stats } = await loader();

		expect(error).toEqual('something went wrong');
		expect(stats).toEqual([]);
	});
});
