import { apiClient } from '$lib/api';
import { authState } from '$lib/stores/auth';
import { toast } from '@zerodevx/svelte-toast';

type LoginData = {
	credentials: string;
};

export const googleCallbackHandler = async (user: LoginData) => {
	const token = user.credential;

	const res = await apiClient.loginWithGoogle(token);

	if (res.status === 200) {
		authState.set(res.data);
		console.info(`Logged in as ${res.data.name}`);
	} else {
		toast.push(`Error ${res.status} getting an authentication token.`);
	}
};
