import { verifyToken } from '$lib/api/users';
import { authStore } from '$lib/stores/authStore';
import { get } from 'svelte/store';

export const validateStoredToken = async () => {
	const auth = get(authStore);

	if (auth === undefined) {
		console.debug('[auth] Not authenticated.');
		return;
	}

	if (auth.token === undefined) {
		console.debug('[auth] No auth token stored.');
		authStore.update(() => undefined);
		return;
	}

	console.debug('[auth] Validating stored auth token...', auth.token);

	const res = await verifyToken(auth.token);

	if (res.status === 401) {
		console.info('[auth] Token expired.');
		authStore.update(() => undefined);
		return;
	}

	console.debug('[auth] Token is OK.');
};
