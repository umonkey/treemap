import type { IUser } from '$lib/types';
import { beforeEach, describe, expect, it, vi } from 'vitest';

const { mockedSetUser } = vi.hoisted(() => ({ mockedSetUser: vi.fn() }));

vi.mock('@sentry/sveltekit', () => ({
	setUser: mockedSetUser
}));

const user: IUser = {
	id: 'user-1',
	name: 'Jane Doe',
	email: 'jane@example.com',
	picture: 'https://example.com/jane.png',
	trees_count: 0,
	comments_count: 0,
	updates_count: 0,
	files_count: 0,
	last_active_at: 0
};

type SentryUserModule = typeof import('./sentryUser');

describe('sentryUser', () => {
	let module: SentryUserModule;

	beforeEach(async () => {
		// A fresh module resets the signature cache between tests.
		vi.resetModules();
		mockedSetUser.mockClear();
		module = await import('./sentryUser');
	});

	it('maps a user to the Sentry identity', () => {
		expect(module.toSentryUser(user)).toEqual({
			id: 'user-1',
			email: 'jane@example.com',
			username: 'Jane Doe'
		});
	});

	it('maps an anonymous user to null', () => {
		expect(module.toSentryUser(undefined)).toBeNull();
	});

	it('sets the Sentry user for an authenticated user', () => {
		module.setSentryUser(user);

		expect(mockedSetUser).toHaveBeenCalledOnce();
		expect(mockedSetUser).toHaveBeenCalledWith({
			id: 'user-1',
			email: 'jane@example.com',
			username: 'Jane Doe'
		});
	});

	it('clears the Sentry user on sign-out', () => {
		module.setSentryUser(user);
		module.setSentryUser(undefined);

		expect(mockedSetUser).toHaveBeenLastCalledWith(null);
	});

	it('skips the scope write when the identity did not change', () => {
		module.setSentryUser(user);
		module.setSentryUser({ ...user });

		expect(mockedSetUser).toHaveBeenCalledOnce();
	});

	it('reports identity changes such as a renamed profile', () => {
		module.setSentryUser(user);
		module.setSentryUser({ ...user, name: 'Jane Smith' });

		expect(mockedSetUser).toHaveBeenCalledTimes(2);
		expect(mockedSetUser).toHaveBeenLastCalledWith({
			id: 'user-1',
			email: 'jane@example.com',
			username: 'Jane Smith'
		});
	});
});
