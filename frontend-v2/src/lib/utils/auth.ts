import { apiClient } from '$lib/api';
import { authState } from '$lib/stores/auth';

export const googleCallbackHandler = async (user) => {
	console.debug('GAUTH', user);

	const token = user.credential;

	const res = await apiClient.loginWithGoogle(token);

	if (res.status === 200) {
		authState.set(res.data);
		console.info(`Logged in as ${res.data.name}`);
	}
};
