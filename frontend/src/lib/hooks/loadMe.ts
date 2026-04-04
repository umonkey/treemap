/**
 * Load information on the current user.
 *
 * TODO: add a store to cache calls.
 */

import { getMe } from '$lib/api/users';
import type { IError, IMeResponse } from '$lib/types';
import { writable } from 'svelte/store';

export const loadMe = () => {
	const loading = writable<boolean>(true);
	const data = writable<IMeResponse | undefined>(undefined);
	const error = writable<IError | undefined>(undefined);
	const statusCode = writable<number | undefined>(undefined);

	const reload = async () => {
		try {
			loading.set(true);

			const { status, data: payload, error: err } = await getMe();

			statusCode.set(status);

			if (status === 200 && payload) {
				data.set(payload);
				error.set(undefined);
			} else {
				data.set(undefined);
				error.set(err);
			}
		} finally {
			loading.set(false);
		}
	};

	return { loading, error, data, statusCode, reload };
};
