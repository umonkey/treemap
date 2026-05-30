import { showError } from '$lib/errors';
import { locale } from '$lib/locale';
import type { ILoginResponse } from '$lib/types';
import { ls } from '$lib/utils/localStorage';
import { derived, writable } from 'svelte/store';

const STORAGE_KEY = 'auth_state';

export const authStore = writable<ILoginResponse | undefined>(ls.read(STORAGE_KEY));

authStore.subscribe((value: ILoginResponse | undefined) => {
	if (!ls.write(STORAGE_KEY, value)) {
		showError(locale.toastStorageError());
	}
});

export const isAuthenticated = derived(authStore, ($authStore) => !!$authStore);
