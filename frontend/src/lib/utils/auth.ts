import { verifyToken } from '$lib/api/users';
import { authStore } from '$lib/stores/authStore';
import * as Sentry from '@sentry/browser';
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

	if (res.data) {
		Sentry.setUser({
			id: res.data.id,
			email: res.data.email,
			username: res.data.name
		});

		console.debug('[Sentry] User id added.');
	}

	console.debug('[auth] Token is OK.');
};
