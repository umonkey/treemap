import { getMe } from '$lib/api/users';
import { DEFAULT_ME } from '$lib/constants';
import type { IMeResponse, IResponse } from '$lib/types';
import { get } from 'svelte/store';
import { describe, expect, it, vi } from 'vitest';
import { loadMe } from './loadMe';

vi.mock('$lib/api/users', () => ({
	getMe: vi.fn()
}));

describe('hooks/loadMe', async () => {
	it('should load a profile', async () => {
		vi.mocked(getMe).mockImplementation(async (): Promise<IResponse<IMeResponse>> => {
			return {
				status: 200,
				data: DEFAULT_ME
			};
		});

		const { loading, error, data, reload } = loadMe();
		expect(get(loading)).toBe(true);

		await reload();

		expect(get(loading)).toBe(false);
		expect(get(error)).toBeUndefined();
		expect(get(data)?.name).toEqual(DEFAULT_ME.name);
	});

	it('should return an error', async () => {
		vi.mocked(getMe).mockImplementation(async (): Promise<IResponse<IMeResponse>> => {
			return {
				status: 500,
				data: undefined,
				error: {
					code: 'SomethingWentWrong',
					description: 'something went wrong'
				}
			};
		});

		const { loading, error, data, reload } = loadMe();
		expect(get(loading)).toBe(true);

		await reload();

		expect(get(loading)).toBe(false);
		expect(get(data)).toBeUndefined();
		expect(get(error)?.description).toEqual('something went wrong');
	});
});
