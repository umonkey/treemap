import type { IMeResponse } from '$lib/types';
import type { Load } from '@sveltejs/kit';
import { apiClient } from '$lib/api';
import { error } from '@sveltejs/kit';

export const load: Load = async (): Promise<{
	profile: IMeResponse;
}> => {
	const res = await apiClient.getMe();

	if (res.status !== 200) {
		error(401);
	}

	return {
		profile: res.data
	};
};
