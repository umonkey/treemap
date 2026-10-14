import { getInstance, updateInstance } from '$lib/api/instance';
import { showError } from '$lib/errors';
import { settingsState } from '$lib/stores/settings.svelte';
import type { IError } from '$lib/types';

export class PageState {
	loading = $state<boolean>(true);
	saving = $state<boolean>(false);
	error = $state<IError | undefined>(undefined);
	name = $state<string>('');
	description = $state<string>('');

	reload = async () => {
		try {
			this.loading = true;

			const res = await getInstance();

			if (res.status === 200 && res.data) {
				this.name = res.data.name;
				this.description = res.data.description ?? '';
				this.error = undefined;
			} else {
				this.error = res.error;
			}
		} finally {
			this.loading = false;
		}
	};

	apply = async () => {
		try {
			this.saving = true;

			const res = await updateInstance({
				name: this.name,
				description: this.description
			});

			if (res.status >= 200 && res.status < 300) {
				settingsState.load();
			} else {
				showError(res.error?.description || `Error ${res.status} saving instance.`);
			}
		} finally {
			this.saving = false;
		}
	};
}
