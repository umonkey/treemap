import type { IUser } from '$lib/types';
import * as Sentry from '@sentry/sveltekit';

let lastSignature: string | undefined;

export const toSentryUser = (user: IUser | undefined) => {
	if (!user) {
		return null;
	}

	return {
		id: user.id,
		email: user.email,
		username: user.name
	};
};

/**
 * Keeps the Sentry user in sync with the application's authenticated user.
 * Passing an undefined user clears the identity, e.g. after signing out.
 */
export const setSentryUser = (user: IUser | undefined) => {
	const signature = user ? `${user.id}|${user.email}|${user.name}` : '';

	// `getMe()` refreshes the user object on the profile pages, so skip the
	// extra scope write when the identity itself did not change.
	if (signature === lastSignature) {
		return;
	}

	lastSignature = signature;

	Sentry.setUser(toSentryUser(user));
};
