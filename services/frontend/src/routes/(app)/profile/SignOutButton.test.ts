import { goto } from '$app/navigation';
import { routes } from '$lib/routes';
import { authStore } from '$lib/stores/authStore';
import type { ILoginResponse } from '$lib/types';
import { cleanup, render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { get } from 'svelte/store';
import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest';
import SignOutButton from './SignOutButton.svelte';

vi.mock('$app/navigation', () => ({
	goto: vi.fn()
}));

const mockedGoto = vi.mocked(goto);

const authenticated: ILoginResponse = {
	token: 'secret',
	user: {
		id: 'user1',
		name: 'John Doe',
		picture: 'https://example.com/picture.jpg',
		email: 'john@example.com',
		trees_count: 0,
		comments_count: 0,
		updates_count: 0,
		files_count: 0,
		last_active_at: 0
	},
	roles: [],
	permissions: []
};

describe('SignOutButton', () => {
	beforeEach(() => {
		mockedGoto.mockClear();
	});

	afterEach(() => {
		cleanup();
		authStore.set(undefined);
	});

	test('renders a non-submitting button', () => {
		render(SignOutButton);

		expect(screen.getByRole('button').getAttribute('type')).toBe('button');
	});

	test('clears the session and navigates home on click', async () => {
		const user = userEvent.setup();
		authStore.set(authenticated);

		render(SignOutButton);

		await user.click(screen.getByRole('button'));

		expect(get(authStore)).toBeUndefined();
		expect(mockedGoto).toHaveBeenCalledWith(routes.home());
	});
});
