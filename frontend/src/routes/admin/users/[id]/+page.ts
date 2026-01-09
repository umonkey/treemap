import { apiClient } from '$lib/api';
import type { PageLoad } from './$types';
import { error } from '@sveltejs/kit';

export const load: PageLoad = async ({ params }) => {
	const res = await apiClient.getUser(params.id);
	if (res.status === 200 && res.data) {
		return {
			user: res.data
		};
	}
	error(404, 'User not found');
};
