import { getUsers } from '$lib/api/users';
import type { PageLoad } from './$types';

export const load: PageLoad = async () => {
	const res = await getUsers();
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
