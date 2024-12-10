import { writable, derived } from 'svelte/store';
import type { ILoginResponse } from '$lib/types';
import { ls } from '$lib/utils/localStorage';

const STORAGE_KEY = 'auth_state';

export const authState = writable(ls.read(STORAGE_KEY));

authState.subscribe((value: ILoginResponse) => {
	ls.write(STORAGE_KEY, value);
});

export const isAuthenticated = derived(authState, ($authState) => !!$authState);
