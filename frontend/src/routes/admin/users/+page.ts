import { apiClient } from '$lib/api';
import type { PageLoad } from './$types';

export const load: PageLoad = async () => {
	const res = await apiClient.getUsers();
	if (res.status === 200 && res.data) {
		return {
			users: res.data.users
		};
	}
	return {
		users: [],
		error: res.error
	};
};
