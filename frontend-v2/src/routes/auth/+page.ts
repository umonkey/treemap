import { apiClient } from '$lib/api';
import { authState } from '$lib/stores/auth';
import { error } from '@sveltejs/kit';

export const load: Load = async ({
	url
}): Promise<{
	redirect: string;
}> => {
	const token = url.searchParams.get('token');
	const state = url.searchParams.get('state');

	console.debug(`[auth] token=${token}`);
	console.debug(`[auth] state=${state}`);

	const res = await apiClient.getMe(token);

	if (res.status !== 200) {
		console.error('[auth] Error fetching user data');
		error(401);
	}

	console.info(`[auth] Logged in as ${res.data.name}`);

	authState.set({
		token,
		name: res.data.name,
		picture: res.data.picture
	});

	return {
		redirect: state
	};
};
