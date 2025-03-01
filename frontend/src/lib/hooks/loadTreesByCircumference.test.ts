import type { IResponse, ITreeList } from '$lib/types';
import { DEFAULT_TREE } from '$lib/constants';
import { apiClient } from '$lib/api';
import { describe, it, expect } from 'vitest';
import { get } from 'svelte/store';
import { loadTreesByCircumference } from './loadTreesByCircumference';

describe('hooks/loadTreesByCircumference', async () => {
	it('should load empty list', async () => {
		apiClient.getTopCircumference = async (): Promise<IResponse<ITreeList>> => {
			return {
				status: 200,
				data: {
					trees: [],
					users: []
				}
			};
		};

		const { loading, error, data, reload } = loadTreesByCircumference();
		expect(get(loading)).toBe(true);

		await reload();

		expect(get(loading)).toBe(false);
		expect(get(error)).toBeUndefined();
		expect(get(data)).toEqual([]);
	});

	it('should load non-empty list', async () => {
		apiClient.getTopCircumference = async (): Promise<IResponse<ITreeList>> => {
			return {
				status: 200,
				data: {
					trees: [DEFAULT_TREE],
					users: []
				}
			};
		};

		const { loading, error, data, reload } = loadTreesByCircumference();
		expect(get(loading)).toBe(true);

		await reload();

		expect(get(loading)).toBe(false);
		expect(get(error)).toBeUndefined();
		expect(get(data).length).toEqual(1);
		expect(get(data)[0].id).toEqual(DEFAULT_TREE.id);
	});

	it('should return an error', async () => {
		apiClient.getTopCircumference = async (): Promise<IResponse<ITreeList>> => {
			return {
				status: 500,
				data: undefined,
				error: {
					code: 'SomethingWentWrong',
					description: 'something went wrong'
				}
			};
		};

		const { loading, error, data, reload } = loadTreesByCircumference();
		expect(get(loading)).toBe(true);

		await reload();

		expect(get(loading)).toBe(false);
		expect(get(data)).toEqual([]);
		expect(get(error)?.description).toEqual('something went wrong');
	});
});
