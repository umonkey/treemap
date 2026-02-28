import { get, writable } from 'svelte/store';
import { apiClient } from '$lib/api';
import { routes, goto } from '$lib/routes';

export const load = (treeId: string) => {
	// Set by the file picker when upload is in progress.
	const busy = writable<boolean>(false);

	// Contains ids of successfully uploaded files.
	const uploads = writable<string[]>([]);

	// Set when the form has files and can be submitted.
	const canSubmit = writable<boolean>(false);

	const hasFiles = writable<boolean>(false);

	const handleBusy = (value: boolean) => {
		busy.set(value);
		canSubmit.set(!value && get(uploads).length > 0);
	};

	const handleChange = (files: number) => {
		hasFiles.set(files > 0);
		busy.set(false);
		canSubmit.set(true);
	};

	const handleSubmit = () => {
		goto(routes.mapPreview(treeId));
	};

	return { canSubmit, handleBusy, handleChange, handleSubmit, hasFiles };
};
