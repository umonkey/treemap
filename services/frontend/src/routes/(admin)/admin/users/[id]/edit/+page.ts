import { getUser } from '$lib/api/users';
import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
	const res = await getUser(params.id);
	if (res.status === 200 && res.data) {
		return {
			user: res.data
		};
	}
	error(404, 'User not found');
};
