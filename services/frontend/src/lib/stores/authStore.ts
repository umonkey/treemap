import { showError } from '$lib/errors';
import { locale } from '$lib/locale';
import type { ILoginResponse } from '$lib/types';
import { ls } from '$lib/utils/localStorage';
import { derived, writable } from 'svelte/store';

const STORAGE_KEY = 'auth_state';

/** Read-only permissions mirrored from the backend's `viewer` role. */
const VIEWER_PERMISSIONS = ['photo:read', 'comment:read'];

export const authStore = writable<ILoginResponse | undefined>(ls.read(STORAGE_KEY));

authStore.subscribe((value: ILoginResponse | undefined) => {
	if (!ls.write(STORAGE_KEY, value)) {
		showError(locale.toastStorageError());
	}
});

export const isAuthenticated = derived(authStore, ($authStore) => !!$authStore);

export const hasPermission = derived(authStore, ($authStore) => {
	return (permission: string) => {
		if (!$authStore) return VIEWER_PERMISSIONS.includes(permission);
		if (!Array.isArray($authStore.permissions)) return false;
		return $authStore.permissions.includes(permission);
	};
});
