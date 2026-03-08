import type { IMeResponse } from '$lib/types';
import { apiClient } from '$lib/api';
import { get, writable } from 'svelte/store';
import { routes, goto } from '$lib/routes';

export const hooks = () => {
	const loading = writable<boolean>(true);
	const saving = writable<boolean>(true);
	const error = writable<string | null>(null);
	const saveError = writable<string | null>(null);
	const data = writable<IMeResponse | null>(null);
	const files = writable<string[]>([]);
	const name = writable<string>('');

	apiClient
		.getMe()
		.then((res) => {
			loading.set(true);
			error.set(null);

			const { status, data: d, error: e } = res;

			if (status === 200 && d) {
				data.set(d);
				name.set(d.name);
			} else if (e) {
				error.set(e.description);
			}
		})
		.finally(() => {
			loading.set(false);
		});

	const handleSave = () => {
		saving.set(true);
		saveError.set(null);

		apiClient
			.updateSettings({
				name: get(name),
				picture: get(files)[0] ?? null
			})
			.then((res) => {
				const { status, error: e } = res;

				if (status === 202) {
					console.info('Profile info updated.');
					goto(routes.profile());
				} else if (e) {
					saveError.set(e.description);
				}
			})
			.finally(() => {
				saving.set(false);
			});
	};

	const handleCancel = () => {
		goto(routes.profile());
	};

	const handleFileBusy = (value: boolean) => {
		saving.set(value);
	};

	const handleFileChange = (value: string[]) => {
		files.set(value);
	};

	const handleNameChange = (value: string) => {
		name.set(value);
	};

	return {
		loading,
		saving,
		error,
		saveError,
		name,
		data,
		handleSave,
		handleCancel,
		handleFileBusy,
		handleFileChange,
		handleNameChange
	};
};
