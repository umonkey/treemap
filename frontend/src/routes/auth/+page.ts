import type { Load } from '@sveltejs/kit';
import { apiClient } from '$lib/api';
import { authStore } from '$lib/stores/authStore';
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

	if (token === null) {
		console.error('[auth] Token not set.');
		error(401);
	}

	if (state === null) {
		console.error('[auth] State not set.');
		error(401);
	}

	const res = await apiClient.verifyToken(token);

	if (res.status === 200 && res.data) {
		authStore.set({
			token,
			name: res.data.name,
			picture: res.data.picture
		});

		return {
			redirect: state
		};
	}

	console.error('[auth] Error fetching user data');
	error(401);
};
