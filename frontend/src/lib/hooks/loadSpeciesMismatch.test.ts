import { apiClient } from '$lib/api';
import { DEFAULT_TREE } from '$lib/constants';
import type { IResponse, ITreeList } from '$lib/types';
import { get } from 'svelte/store';
import { describe, expect, it } from 'vitest';
import { loadSpeciesMismatch } from './loadSpeciesMismatch';

describe('hooks/loadSpeciesMismatch', async () => {
	it('should load empty list', async () => {
		apiClient.getSpeciesMismatch = async (): Promise<IResponse<ITreeList>> => {
			return {
				status: 200,
				data: {
					trees: [],
					users: []
				}
			};
		};

		const { loading, error, data, reload } = loadSpeciesMismatch();
		expect(get(loading)).toBe(true);

		await reload();

		expect(get(loading)).toBe(false);
		expect(get(error)).toBeUndefined();
		expect(get(data)).toEqual([]);
	});

	it('should load non-empty list', async () => {
		apiClient.getSpeciesMismatch = async (): Promise<IResponse<ITreeList>> => {
			return {
				status: 200,
				data: {
					trees: [DEFAULT_TREE],
					users: []
				}
			};
		};

		const { loading, error, data, reload } = loadSpeciesMismatch();
		expect(get(loading)).toBe(true);

		await reload();

		expect(get(loading)).toBe(false);
		expect(get(error)).toBeUndefined();
		expect(get(data).length).toEqual(1);
		expect(get(data)[0].id).toEqual(DEFAULT_TREE.id);
	});

	it('should return an error', async () => {
		apiClient.getSpeciesMismatch = async (): Promise<IResponse<ITreeList>> => {
			return {
				status: 500,
				data: undefined,
				error: {
					code: 'SomethingWentWrong',
					description: 'something went wrong'
				}
			};
		};

		const { loading, error, data, reload } = loadSpeciesMismatch();
		expect(get(loading)).toBe(true);

		await reload();

		expect(get(loading)).toBe(false);
		expect(get(data)).toEqual([]);
		expect(get(error)?.description).toEqual('something went wrong');
	});
});
