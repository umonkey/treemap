import { showError } from '$lib/errors';
import { locale } from '$lib/locale';
import { ls } from '$lib/utils/localStorage';
import { writable } from 'svelte/store';

const STORAGE_KEY = 'app_settings';

interface ISettings {
	keepAwake: boolean;
}

const DEFAULT_SETTINGS: ISettings = {
	keepAwake: true
};

const getInitialSettings = (): ISettings => {
	const stored = ls.read<ISettings>(STORAGE_KEY);
	if (stored) {
		return { ...DEFAULT_SETTINGS, ...stored };
	}
	return DEFAULT_SETTINGS;
};

export const settingsStore = writable<ISettings>(getInitialSettings());

settingsStore.subscribe((value) => {
	if (!ls.write(STORAGE_KEY, value)) {
		showError(locale.toastStorageError());
	}
});
